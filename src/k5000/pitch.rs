//! Data model for the pitch envelope.
//!

use std::fmt;

use rand::RngExt;
use syxpack::{
    SystemExclusiveData,
    ParseError,
    Ranged,
    ranged_impl,
    Encoding,
    parse_or_default,
};

/// Velocity sensitivity (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocitySensitivity(i32);
ranged_impl!(VelocitySensitivity, -63, 63, 0);

impl Encoding for VelocitySensitivity {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// Pitch envelope level (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
ranged_impl!(EnvelopeLevel, -63, 63, 0);

impl Encoding for EnvelopeLevel {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// Pitch envelope time (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
ranged_impl!(EnvelopeTime, 0, 127, 0);

impl Encoding for EnvelopeTime { }

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
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            start: parse_or_default::<EnvelopeLevel>(data[0]),
            attack_time: parse_or_default::<EnvelopeTime>(data[1]),
            attack_level: parse_or_default::<EnvelopeLevel>(data[2]),
            decay_time: parse_or_default::<EnvelopeTime>(data[3]),
            time_vel_sens: parse_or_default::<VelocitySensitivity>(data[4]),
            level_vel_sens: parse_or_default::<VelocitySensitivity>(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.start.encode(),
            self.attack_time.encode(),
            self.attack_level.encode(),
            self.decay_time.encode(),
            self.time_vel_sens.encode(),
            self.level_vel_sens.encode(),
        ]
    }

    fn data_size() -> usize { 6 }
}
