//! Data model for the pitch envelope.
//!

use std::fmt;

use rand::RngExt;
use syxpack::{
    SystemExclusiveData,
    ParseError,
    Ranged,
    ranged_impl,
};

/// Velocity sensitivity (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocitySensitivity(i32);
ranged_impl!(VelocitySensitivity, -63, 63, 0);

impl From<u8> for VelocitySensitivity {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for VelocitySensitivity {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

/// Pitch envelope level (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
ranged_impl!(EnvelopeLevel, -63, 63, 0);

impl From<u8> for EnvelopeLevel {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for EnvelopeLevel {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

/// Pitch envelope time (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
ranged_impl!(EnvelopeTime, 0, 127, 0);

impl From<u8> for EnvelopeTime {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl Into<u8> for EnvelopeTime {
    fn into(self) -> u8 {
        self.value() as u8
    }
}

/// Pitch envelope.
#[derive(Debug)]
pub struct Envelope {
    /// Envelope start level.
    pub start: EnvelopeLevel,

    /// Envelope attack time.
    pub attack_time: EnvelopeTime,

    /// Envelope attack level.
    pub attack_level: EnvelopeLevel,

    /// Envelope decay time.
    pub decay_time: EnvelopeTime,

    /// Time velocity sensitivity.
    pub time_vel_sens: VelocitySensitivity,

    /// Level velocity sensitivity.
    pub level_vel_sens: VelocitySensitivity,
}

impl Envelope {
    /// Creates a new envelope with default values.
    pub fn new() -> Envelope {
        Self {
            start: EnvelopeLevel::new(0),
            attack_time: EnvelopeTime::new(0),
            attack_level: EnvelopeLevel::new(0),
            decay_time: EnvelopeTime::new(0),
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
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            start: EnvelopeLevel::from(data[0]),
            attack_time: EnvelopeTime::from(data[1]),
            attack_level: EnvelopeLevel::from(data[2]),
            decay_time: EnvelopeTime::from(data[3]),
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

    fn data_size() -> usize { 6 }
}
