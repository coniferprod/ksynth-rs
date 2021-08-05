//! Data model for a PCM source oscillator.
//!

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::StringUtils;
use crate::SystemExclusiveData;
use crate::k5000::pitch::Envelope as PitchEnvelope;

/// PCM oscillator.
pub struct Oscillator {
    pub wave: u16,
    pub coarse: i8,
    pub fine: i8,
    pub ks_to_pitch: KeyScaling,
    pub fixed_key: u8,
    pub pitch_envelope: PitchEnvelope,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            wave: 384,
            coarse: 0,
            fine: 0,
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: 60,
            pitch_envelope: PitchEnvelope::new(),
        }
    }

    pub fn additive() -> Oscillator {
        Oscillator {
            wave: 512, // ADD
            coarse: 0,
            fine: 0,
            ks_to_pitch: KeyScaling::ZeroCent,
            fixed_key: 60,
            pitch_envelope: PitchEnvelope::new(),
        }
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Oscillator::new()
    }
}

impl SystemExclusiveData for Oscillator {
    fn from_bytes(data: Vec<u8>) -> Self {
        Oscillator {
            wave: 384,  // TODO: actually parse wave number
            coarse: (data[2] - 24) as i8,
            fine: (data[3] - 64) as i8,
            ks_to_pitch: KeyScaling::try_from(data[4]).unwrap(),
            fixed_key: data[5],
            pitch_envelope: PitchEnvelope::from_bytes(data[6..].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        // Convert wave number to 10-bit binary string
        let s = format!("{:010b}", self.wave);
        let msb_s = s.substring(0, 3);
        let lsb_s = s.substring(3, 7);

        let msb = u8::from_str_radix(&msb_s, 2).unwrap();
        let lsb = u8::from_str_radix(&lsb_s, 2).unwrap();

        result.push(msb);
        result.push(lsb);

        let bs = vec![
            (self.coarse + 24) as u8,
            (self.fine + 64) as u8,
            self.fixed_key,
            self.ks_to_pitch as u8,
        ];
        result.extend(bs);

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
