use std::fmt;

use rand::RngExt;
use syxpack::{
    Ranged, 
    ranged_impl,
    Encoding,
};

pub mod amp;
pub mod effect;
pub mod filter;
pub mod lfo;
pub mod multi;
pub mod single;
pub mod source;
pub mod wave;
pub mod drum;
pub mod bank;
pub mod sysex;

/// Length of patch name
pub const NAME_LENGTH: usize = 10;

/// Number of sources in a single patch
pub const SOURCE_COUNT: usize = 4;

/// Number of DRUM notes
pub const DRUM_NOTE_COUNT: usize = 61;

/// Number of submix channels / outputs
pub const SUBMIX_COUNT: usize = 8;

fn get_effect_number(b: u8) -> u8 {
    let value = b & 0b00011111;
    // Now we should have a value in the range 0~31.
    // Use range 1~32 when storing the value.
    value + 1
}

/// Envelope time for DCA/DCF attack, decay and release
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
ranged_impl!(EnvelopeTime, 0, 100, 0);
impl Encoding for EnvelopeTime { }

/// Envelope level for DCA/DCF sustain (level from 0 to 100)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
ranged_impl!(EnvelopeLevel, 0, 100, 0);
impl Encoding for EnvelopeLevel { }

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Level(i32);
ranged_impl!(Level, 0, 100, 0);
impl Encoding for Level { }

/// Depth used for DCA/DCF modulation values
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ModulationDepth(i32);
ranged_impl!(ModulationDepth, -50, 50, 0);

impl Encoding for ModulationDepth {
    fn decode(b: u8) -> i32 {
        (b as i32) - 50
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 50
    }
}

/// Drum source 1 and 2 decay
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Decay(i32);
ranged_impl!(Decay, 1, 100, 0);
impl Encoding for Decay { }

/// Small effect parameter
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SmallEffectParameter(i32);
ranged_impl!(SmallEffectParameter, -7, 7, 0);

impl Encoding for SmallEffectParameter {
    fn decode(b: u8) -> i32 {
        (b as i32) - 7
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 7
    }
}

/// Big effect parameter
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BigEffectParameter(i32);
ranged_impl!(BigEffectParameter, 0, 31, 0);
impl Encoding for BigEffectParameter { }

/// Envelope level for DCF sustain
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FilterEnvelopeLevel(i32);
ranged_impl!(FilterEnvelopeLevel, -50, 50, 0);

impl Encoding for FilterEnvelopeLevel {
    fn decode(b: u8) -> i32 {
        (b as i32) - 50
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 50
    }
}

/// Filter cutoff
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cutoff(i32);
ranged_impl!(Cutoff, 0, 100, 0);
impl Encoding for Cutoff { }

/// Filter resonance
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Resonance(i32);
ranged_impl!(Resonance, 0, 7, 0);
impl Encoding for Resonance { }

/// Effect number
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectNumber(i32);
ranged_impl!(EffectNumber, 1, 32, 0);

impl Encoding for EffectNumber {
    fn decode(b: u8) -> i32 {
        (b as i32) + 1
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) - 1
    }
}

/// Velocity or Key Scaling curve 1~8
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Curve(i32);
ranged_impl!(Curve, 1, 8, 1);

impl Encoding for Curve {
    fn decode(b: u8) -> i32 {
        (b as i32) + 1
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) - 1
    }
}

/// DCO coarse tuning
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coarse(i32);
ranged_impl!(Coarse, -24, 24, 0);

impl Encoding for Coarse {
    fn decode(b: u8) -> i32 {
        (b as i32) - 24
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 24
    }
}

/// DCO fine tuning
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fine(i32);
ranged_impl!(Fine, -50, 50, 0);

impl Encoding for Fine {
    fn decode(b: u8) -> i32 {
        (b as i32) - 50
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 50
    }
}

/// Wave number
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct WaveNumber(i32);
ranged_impl!(WaveNumber, 1, 256, 0);

/// Patch number 0...63 (can be converted to A-1...D-16)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PatchNumber(i32);
ranged_impl!(PatchNumber, 0, 63, 0);

/// Transpose
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Transpose(i32);
ranged_impl!(Transpose, -24, 24, 0); // +-24 (in SysEx 0~48)

impl Encoding for Transpose {
    fn decode(b: u8) -> i32 {
        (b as i32) - 24
    }

    fn encode(&self) -> u8 {
        (self.value() as u8) + 24
    }
}
