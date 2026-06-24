use std::fmt;
use rand::Rng;

use crate::{Ranged, ranged_impl};

pub mod filter;
pub mod amp;
pub mod osc;
pub mod pitch;
pub mod lfo;
pub mod control;
pub mod source;
pub mod effect;
pub mod single;
pub mod multi;
pub mod morf;
pub mod harmonic;
pub mod formant;
pub mod addkit;
pub mod wave;
pub mod sysex;

/// Length of patch name
pub const NAME_LENGTH: usize = 8;

/// Volume of patch (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Volume(i32);
ranged_impl!(Volume, 0, 127, 0);

impl From<u8> for Volume {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl Into<u8> for Volume {
    fn into(self) -> u8 {
        self.value() as u8
    }
}

/// Envelope time (0...127, default 0).
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

/// Envelope level (-63...63, default 0).
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

/// Envelope rate (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeRate(i32);
ranged_impl!(EnvelopeRate, 0, 127, 0);

impl From<u8> for EnvelopeRate {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl Into<u8> for EnvelopeRate {
    fn into(self) -> u8 {
        self.value() as u8
    }
}

/// Control time (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct ControlTime(i32);
ranged_impl!(ControlTime, -63, 63, 0);

impl From<u8> for ControlTime {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for ControlTime {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

/// Envelope depth (-63...63, default 0).
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeDepth(i32);
ranged_impl!(EnvelopeDepth, -63, 63, 0);

impl From<u8> for EnvelopeDepth {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for EnvelopeDepth {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

/// Depth (0...100, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Depth(i32);
ranged_impl!(Depth, 0, 100, 0);

impl From<u8> for Depth {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl Into<u8> for Depth {
    fn into(self) -> u8 {
        self.value() as u8
    }
}

/// Key scaling (-63...63, default 0)
/// SysEx storage: one byte, (-63)1~(+63)127.
/// Adjustment: incoming -64, outgoing +64. 
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyScaling(i32);
ranged_impl!(KeyScaling, -63, 63, 0);

impl From<u8> for KeyScaling {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl Into<u8> for KeyScaling {
    fn into(self) -> u8 {
        (self.value() + 64) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{*};
}
