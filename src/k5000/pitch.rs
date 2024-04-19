//! Data model for the pitch envelope.
//!

use std::fmt;

use crate::{SystemExclusiveData, ParseError};
use crate::k5000::{PitchEnvelopeLevel, PitchEnvelopeTime, VelocitySensitivity};

/// Pitch envelope.
pub struct Envelope {
    /// Envelope start level.
    pub start: PitchEnvelopeLevel,

    /// Envelope attack time.
    pub attack_time: PitchEnvelopeTime,

    /// Envelope attack level.
    pub attack_level: PitchEnvelopeLevel,

    /// Envelope decay time.
    pub decay_time: PitchEnvelopeTime,

    /// Time velocity sensitivity.
    pub time_vel_sens: VelocitySensitivity,

    /// Level velocity sensitivity.
    pub level_vel_sens: VelocitySensitivity,
}

impl Envelope {
    /// Creates a new envelope with default values.
    pub fn new() -> Envelope {
        Envelope {
            start: PitchEnvelopeLevel::new(0),
            attack_time: PitchEnvelopeTime::new(0),
            attack_level: PitchEnvelopeLevel::new(0),
            decay_time: PitchEnvelopeTime::new(0),
            time_vel_sens: VelocitySensitivity::new(0),
            level_vel_sens: VelocitySensitivity::new(0),
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
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(Envelope {
            start: PitchEnvelopeLevel::from(data[0]),
            attack_time: PitchEnvelopeTime::from(data[1]),
            attack_level: PitchEnvelopeLevel::from(data[2]),
            decay_time: PitchEnvelopeTime::from(data[3]),
            time_vel_sens: VelocitySensitivity::from(data[4]),
            level_vel_sens: VelocitySensitivity::from(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.start.into(),
            self.attack_time.into(),
            self.attack_level.into(),
            self.decay_time.into(),
            self.time_vel_sens.into(),
            self.level_vel_sens.into()
        ]
    }
}
