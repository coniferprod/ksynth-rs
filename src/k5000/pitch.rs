//! Data model for the pitch envelope.
//!

use std::fmt;

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
            start: Level::new(0),
            attack_time: Time::new(0),
            attack_level: Level::new(0),
            decay_time: Time::new(0),
            time_vel_sens: VelocitySensitivity::new(0),
            level_vel_sens: VelocitySensitivity::new(0),
        }
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "                  Pitch Envelope\nStrt L  {}       Vel to\nAtak T  {}     Level {}\nAtak L  {}     Time  {}\nDecy T  {}\n",
            self.start, self.attack_time, self.level_vel_sens, self.attack_level, self.time_vel_sens, self.decay_time
        )
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
