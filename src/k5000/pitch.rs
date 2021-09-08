//! Data model for the pitch envelope.
//!

use crate::SystemExclusiveData;
use crate::k5000::{SignedLevel, UnsignedLevel};

/// Envelope level.
pub type Level = SignedLevel;

/// Envelope time.
pub type Time = UnsignedLevel;

/// Velocity sensitivity.
pub type VelocitySensitivity = SignedLevel;

/// Pitch envelope.
pub struct Envelope {
    /// Envelope start level.
    pub start: Level,

    /// Envelope attack time.
    pub attack_time: Time,

    /// Envelope attack level.
    pub attack_level: Level,

    /// Envelope decay time.
    pub decay_time: Time,

    /// Time velocity sensitivity.
    pub time_vel_sens: VelocitySensitivity,

    /// Level velocity sensitivity.
    pub level_vel_sens: VelocitySensitivity,
}

impl Envelope {
    /// Creates a new envelope with default values.
    pub fn new() -> Envelope {
        Envelope {
            start: Level::from(0i8),
            attack_time: Time::from(0),
            attack_level: Level::from(0i8),
            decay_time: Time::from(0),
            time_vel_sens: VelocitySensitivity::from(0i8),
            level_vel_sens: VelocitySensitivity::from(0i8),
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            start: Level::from(data[0]),
            attack_time: Time::from(data[1]),
            attack_level: Level::from(data[2]),
            decay_time: Time::from(data[3]),
            time_vel_sens: VelocitySensitivity::from(data[4]),
            level_vel_sens: VelocitySensitivity::from(data[5]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let bs = vec![
            self.start.as_byte(),
            self.attack_time.as_byte(),
            self.attack_level.as_byte(),
            self.decay_time.as_byte(),
            self.time_vel_sens.as_byte(),
            self.level_vel_sens.as_byte()
        ];
        result.extend(bs);
        result
    }
}
