//! Data model for the pitch envelope.
//!

use std::fmt;

use serde::{Serialize, Deserialize};

use crate::{
    SystemExclusiveData,
    ParseError,
};
use crate::k5000::{
    ByteValue,
    DESCRIPTORS,
};

/// Pitch envelope.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    /// Envelope start level.
    pub start: i32, // PitchEnvelopeLevel: -63...63, default 0

    /// Envelope attack time.
    pub attack_time: i32,  // PitchEnvelopeTime: 0...127, default 0

    /// Envelope attack level.
    pub attack_level: i32, // PitchEnvelopeLevel: -63...63, default 0

    /// Envelope decay time.
    pub decay_time: i32,  // PitchEnvelopeTime: 0...127, default 0

    /// Time velocity sensitivity.
    pub time_vel_sens: i32,  // PitchEnvelopeLevel: -63...63, default 0

    /// Level velocity sensitivity.
    pub level_vel_sens: i32,  // PitchEnvelopeLevel: -63...63, default 0
}

impl Envelope {
    /// Creates a new envelope with default values.
    pub fn new() -> Envelope {
        Self {
            start: Default::default(),
            attack_time: Default::default(),
            attack_level: Default::default(),
            decay_time: Default::default(),
            time_vel_sens: Default::default(),
            level_vel_sens: Default::default(),
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start Level={} Attack Time={} Attack Level={} Decay Time={}\nVelocity to: Level={} Time={}\n",
            self.start, self.attack_time, self.attack_level, self.decay_time, self.level_vel_sens, self.time_vel_sens
        )
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let time_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeTime).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeLevel).unwrap();
        let vs_desc = DESCRIPTORS.get(&ByteValue::VelocitySensitivity).unwrap();

        Ok(Self {
            start: (level_desc.incoming)(data[0]),
            attack_time: (time_desc.incoming)(data[1]),
            attack_level: (level_desc.incoming)(data[2]),
            decay_time: (time_desc.incoming)(data[3]),
            time_vel_sens:(vs_desc.incoming)(data[4]),
            level_vel_sens: (vs_desc.incoming)(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let time_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeTime).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeLevel).unwrap();
        let vs_desc = DESCRIPTORS.get(&ByteValue::VelocitySensitivity).unwrap();

        vec![
            (level_desc.outgoing)(self.start),
            (time_desc.outgoing)(self.attack_time),
            (level_desc.outgoing)(self.attack_level),
            (time_desc.outgoing)(self.decay_time),
            (vs_desc.outgoing)(self.time_vel_sens),
            (vs_desc.outgoing)(self.level_vel_sens)
        ]
    }

    fn data_size() -> usize { 6 }
}
