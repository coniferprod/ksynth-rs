//! Data model for a PCM source oscillator.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use pretty_hex::*;
use rand::Rng;

use crate::{
    Adjustment,
    MIDINote, 
    ParseError, 
    Ranged, 
    SystemExclusiveData, 
    parse_or_default, 
    ranged_impl,
};
use crate::k5000::pitch::Envelope as PitchEnvelope;
use crate::k5000::wave::Wave;
use crate::k5000::source::Key;

/// Coarse (-24...24, default 0).
/// SysEx storage: one byte, (-24)40~(+24)88.
/// Adjustment: incoming -64, outgoing +64.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coarse(i32);
ranged_impl!(Coarse, -24, 24, 0);

impl From<u8> for Coarse {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for Coarse {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

impl Adjustment for Coarse {
    fn incoming(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn outgoing(&self) -> u8 {
        (self.value() + 64) as u8
    }
}

/// Fine (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fine(i32);
ranged_impl!(Fine, -63, 63, 0);

impl From<u8> for Fine {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for Fine {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

impl Adjustment for Fine {
    fn incoming(b: u8) -> i32 {
        (b as i32) - 64        
    }

    fn outgoing(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// Fixed key for oscillator.
#[derive(Debug)]
pub enum FixedKey {
    Off,
    On(Key)
}

impl fmt::Display for FixedKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FixedKey::Off => String::from("OFF"),
            FixedKey::On(key) => key.name(),
        };
        write!(f, "{}", &s)
    }
}

impl SystemExclusiveData for FixedKey {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        if data[0] == 0x00 {
            Ok(FixedKey::Off)
        }
        else {
            Ok(FixedKey::On(Key { note: MIDINote::from(data[0] - 21) }))
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            FixedKey::Off => vec![0x00],
            FixedKey::On(key) => {
                let b: u8 = key.note.into();
                vec![b + 21]
            },
        }
    }

    fn data_size() -> usize { 1 }
}

/// PCM oscillator.
#[derive(Debug)]
pub struct Oscillator {
    pub wave: Wave,
    pub coarse: Coarse,
    pub fine: Fine,
    pub ks_to_pitch: KeyScaling,
    pub fixed_key: FixedKey,
    pub pitch_envelope: PitchEnvelope,
}

impl Oscillator {
    /// Makes a new oscillator with default values for PCM.
    pub fn new() -> Oscillator {
        Self {
            wave: Wave { number: 384 },
            coarse: Default::default(),
            fine: Default::default(),
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: FixedKey::Off,
            pitch_envelope: PitchEnvelope::new(),
        }
    }

    /// Makes a new oscillator with default values for ADD.
    pub fn additive() -> Oscillator {
        Self {
            wave: Wave { number: 512 }, // ADD
            coarse: Default::default(),
            fine: Default::default(),
            fixed_key: FixedKey::Off,
            ks_to_pitch: KeyScaling::ZeroCent,
            pitch_envelope: PitchEnvelope::new(),
        }
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Oscillator::new()
    }
}

impl fmt::Display for Oscillator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wave={}\nKS Pitch={}\nFixed Key={}\nCoarse={} Fine={}\nPitch Envelope: {}",
            self.wave, self.ks_to_pitch, self.fixed_key, self.coarse, self.fine, self.pitch_envelope)
    }
}

impl SystemExclusiveData for Oscillator {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("OSC data = {}", simple_hex(&data));

        Ok(Oscillator {
            wave: Wave::from_bytes(&[data[0], data[1]])?,
            coarse: parse_or_default::<Coarse>(data[2]),
            fine: parse_or_default::<Fine>(data[3]),
            fixed_key: FixedKey::from_bytes(&[data[4]])?,
            ks_to_pitch: KeyScaling::try_from(data[5]).unwrap(),
            pitch_envelope: PitchEnvelope::from_bytes(&data[6..])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.wave.to_bytes());
        result.push(self.coarse.outgoing());
        result.push(self.fine.outgoing());
        result.extend(self.fixed_key.to_bytes());
        result.push(self.ks_to_pitch as u8);
        result.extend(self.pitch_envelope.to_bytes());

        result
    }

    fn data_size() -> usize {
        Wave::data_size()
        + 4
        + PitchEnvelope::data_size()
    }
}

/// Key scaling type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum KeyScaling {
    ZeroCent = 0,
    TwentyFiveCent = 1,
    ThirtyTreeCent = 2,
    FiftyCent = 3,
}

impl fmt::Display for KeyScaling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            KeyScaling::ZeroCent => String::from("0ct"),
            KeyScaling::TwentyFiveCent => String::from("25ct"),
            KeyScaling::ThirtyTreeCent => String::from("33ct"),
            KeyScaling::FiftyCent => String::from("50ct")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_parse_or_default_coarse_max() {
        //let b = 100;  // invalid, SysEx should be 40...88
        let coarse = parse_or_default::<Coarse>(88u8);
        assert_eq!(coarse.value(), 24);
    }

    #[test]
    fn test_parse_or_default_coarse_min() {
        let coarse = parse_or_default::<Coarse>(40u8);
        assert_eq!(coarse.value(), -24);
    }

    #[test]
    fn test_parse_or_default_coarse_invalid() {
        let b = 100;  // invalid, SysEx byte should be 40...88
        let coarse = parse_or_default::<Coarse>(100u8);
        assert_eq!(coarse.value(), 0);  // should revert to default
    }
}
