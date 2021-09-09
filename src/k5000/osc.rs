//! Data model for a PCM source oscillator.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;

use crate::SystemExclusiveData;
use crate::k5000::pitch::Envelope as PitchEnvelope;
use crate::k5000::{Coarse, SignedLevel};
use crate::k5000::wave::Wave;
use crate::k5000::source::Key;

pub type Fine = SignedLevel;

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

        // Works, but duplicates the write! macro:
        /*
        match self {
            FixedKey::Off => write!(f, "{}", "OFF"),
            FixedKey::On(key) => write!(f, "{}", key.name()),
        }
        */

        // Borrow checker trouble:
        /*
        write!(f, "{}", match self {
            FixedKey::Off => "OFF",
            FixedKey::On(key) => key.name().as_str(),
        })
        */
    }
}

impl SystemExclusiveData for FixedKey {
    fn from_bytes(data: Vec<u8>) -> Self {
        if data[0] == 0x00 {
            FixedKey::Off
        }
        else {
            FixedKey::On(Key { note: data[0] - 21 })
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
            coarse: Coarse::from(0),
            fine: Fine::from(0),
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: FixedKey::Off,
            pitch_envelope: PitchEnvelope::new(),
        }
    }

    /// Makes a new oscillator with default values for ADD.
    pub fn additive() -> Oscillator {
        Oscillator {
            wave: Wave { number: 512 }, // ADD
            coarse: Coarse::from(0),
            fine: Fine::from(0),
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: FixedKey::Off,
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
        write!(f, "Wave type: {}\nKS Pitch: {:?}\nFixed key: {}\nCoarse: {:?}  Fine: {:?}\nEnvelope:\n{}\n",
            self.wave, self.ks_to_pitch, self.fixed_key, self.coarse, self.fine, self.pitch_envelope)
    }
}

impl SystemExclusiveData for Oscillator {
    fn from_bytes(data: Vec<u8>) -> Self {
        Oscillator {
            wave: Wave::from_bytes(vec![data[0], data[1]]),
            coarse: Coarse::from(data[2]),
            fine: Fine::from(data[3]),
            ks_to_pitch: KeyScaling::try_from(data[4]).unwrap(),
            fixed_key: FixedKey::from_bytes(vec![data[5]]),
            pitch_envelope: PitchEnvelope::from_bytes(data[6..].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.wave.to_bytes());
        result.push(self.coarse.as_byte());
        result.push(self.fine.as_byte());
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
