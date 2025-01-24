use std::fmt;

use rand::Rng;

use crate::{
    SystemExclusiveData,
    ParseError,
    Ranged,
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

pub fn get_note_name(note_number: u8) -> String {
    let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ];
    let octave = (note_number / 12) - 2;
    let name = notes[note_number as usize % 12];

    format!("{}{}", name, octave)
}

/// Envelope time for DCA/DCF attack, decay and release
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
crate::ranged_impl!(EnvelopeTime, 0, 100, 0);

/// Envelope level for DCA/DCF sustain
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
crate::ranged_impl!(EnvelopeLevel, 0, 100, 0);

/// Level from 0 to 100
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Level(i32);
crate::ranged_impl!(Level, 0, 100, 0);

/// Depth used for DCA/DCF modulation values
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ModulationDepth(i32);
crate::ranged_impl!(ModulationDepth, -50, 50, 0);

/// Drum source 1 and 2 decay
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Decay(i32);
crate::ranged_impl!(Decay, 1, 100, 1);

/// Small effect parameter
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SmallEffectParameter(i32);
crate::ranged_impl!(SmallEffectParameter, -7, 7, 0);

/// Big effect parameter
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BigEffectParameter(i32);
crate::ranged_impl!(BigEffectParameter, 0, 31, 0);

/// Envelope level for DCF sustain
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FilterEnvelopeLevel(i32);
crate::ranged_impl!(FilterEnvelopeLevel, -50, 50, 0);

/// Filter cutoff
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cutoff(i32);
crate::ranged_impl!(Cutoff, 0, 100, 0);

/// Filter resonance
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Resonance(i32);
crate::ranged_impl!(Resonance, 0, 7, 0);

/// Effect number
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectNumber(i32);
crate::ranged_impl!(EffectNumber, 1, 32, 1);

impl SystemExclusiveData for EffectNumber {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self::new((data[0] + 1) as i32))  // adjust 0~31 to 1~32
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.value() as u8 - 1]
    }

    fn data_size() -> usize { 1 }
}

/// Velocity or Key Scaling curve 1~8
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Curve(i32);
crate::ranged_impl!(Curve, 1, 8, 1);

/// DCO coarse tuning
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coarse(i32);
crate::ranged_impl!(Coarse, -24, 24, 0);

/// DCO fine tuning
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fine(i32);
crate::ranged_impl!(Fine, -50, 50, 0);

/// Wave number
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct WaveNumber(i32);
crate::ranged_impl!(WaveNumber, 1, 256, 1);

/// Patch number 0...63 (can be converted to A-1...D-16)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PatchNumber(i32);
crate::ranged_impl!(PatchNumber, 0, 63, 0);

/// Transpose
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Transpose(i32);
crate::ranged_impl!(Transpose, -24, 24, 0);

impl SystemExclusiveData for Transpose {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self::new((data[0] as i32) - 24))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self.value() + 24) as u8]
    }

    fn data_size() -> usize { 1 }
}
