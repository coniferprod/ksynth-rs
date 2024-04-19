//! Data model for a PCM source oscillator.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use pretty_hex::*;

use crate::{
    SystemExclusiveData, 
    ParseError
};
use crate::k5000::pitch::Envelope as PitchEnvelope;
use crate::k5000::{
    Coarse, 
    Fine
};
use crate::k5000::wave::Wave;
use crate::k5000::source::Key;

/// Fixed key for oscillator.
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
            Ok(FixedKey::On(Key { note: data[0] - 21 }))
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            FixedKey::Off => vec![0x00],
            FixedKey::On(key) => vec![key.note + 21],
        }
    }
}

/// PCM oscillator.
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
        Oscillator {
            wave: Wave { number: 384 },
            coarse: Coarse::new(0),
            fine: Fine::new(0),
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: FixedKey::Off,
            pitch_envelope: PitchEnvelope::new(),
        }
    }

    /// Makes a new oscillator with default values for ADD.
    pub fn additive() -> Oscillator {
        Oscillator {
            wave: Wave { number: 512 }, // ADD
            coarse: Coarse::new(0),
            fine: Fine::new(0),
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
            coarse: Coarse::from(data[2]),
            fine: Fine::from(data[3]),
            fixed_key: FixedKey::from_bytes(&[data[4]])?,
            ks_to_pitch: KeyScaling::try_from(data[5]).unwrap(),
            pitch_envelope: PitchEnvelope::from_bytes(&data[6..])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.wave.to_bytes());
        result.push(self.coarse.into());
        result.push(self.fine.into());
        result.extend(self.fixed_key.to_bytes());
        result.push(self.ks_to_pitch as u8);
        result.extend(self.pitch_envelope.to_bytes());

        result
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
