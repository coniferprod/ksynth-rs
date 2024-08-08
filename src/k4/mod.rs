use crate::{SystemExclusiveData, ParseError};

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

// Domain types based on nutype.
// We use the smallest possible inner type for the wrapped value, FWIW.

use nutype::nutype;

/// Envelope time for DCA/DCF attack, decay and release
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 100),
    derive(Copy, Clone, PartialEq, Eq)
)]
pub struct EnvelopeTime(u8);

/// Envelope level for DCA/DCF sustain
type EnvelopeLevel = EnvelopeTime;

/// Level from 0 to 100
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 100),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Level(u8);

/// Depth used for DCA/DCF modulation values
#[nutype(
    validate(greater_or_equal = -50, less_or_equal = 50),
    derive(Copy, Clone)
)]
pub struct ModulationDepth(i8);  // note: signed inner type

/// MIDI channel
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 16),
    derive(Copy, Clone)
)]
pub struct Channel(u8);

/// Drum source 1 and 2 decay
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 100),
    derive(Copy, Clone)
)]
pub struct Decay(u8);

/// Small effect parameter
#[nutype(
    validate(greater_or_equal = -7, less_or_equal = 7),
    derive(Copy, Clone)
)]
pub struct SmallEffectParameter(i8);

/// Big effect parameter
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 31),
    derive(Copy, Clone)
)]
pub struct BigEffectParameter(u8);

/// Envelope level for DCF sustain
#[nutype(
    validate(greater_or_equal = -50, less_or_equal = 50),
    derive(Copy, Clone, PartialEq, Eq)
)]
pub struct FilterEnvelopeLevel(i8);

/// Filter cutoff
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 100),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Cutoff(u8);

/// Filter resonance
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 7),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Resonance(u8);

/// Effect number
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 32),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct EffectNumber(u8);

impl SystemExclusiveData for EffectNumber {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self::try_new(data[0] + 1).unwrap())  // adjust 0~31 to 1~32
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.into_inner() - 1]
    }

    fn data_size() -> usize { 1 }
}

/// Velocity or Key Scaling curve 1~8
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 8),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Curve(u8);

/// DCO coarse tuning
#[nutype(
    validate(greater_or_equal = -24, less_or_equal = 24),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Coarse(i8);

/// DCO fine tuning
#[nutype(
    validate(greater_or_equal = -50, less_or_equal = 50),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Fine(i8);

/// Wave number
#[nutype(
    validate(greater_or_equal = 1, less_or_equal = 256),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct WaveNumber(u16);

/// Patch number 0...63 (can be converted to A-1...D-16)
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 63),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct PatchNumber(u8);

/// Transpose
#[nutype(
    validate(greater_or_equal = -24, less_or_equal = 24), // +-24 (in SysEx 0~48)
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct Transpose(i8);

impl SystemExclusiveData for Transpose {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self::try_new(data[0] as i8 - 24).unwrap())
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self.into_inner() + 24) as u8]
    }

    fn data_size() -> usize { 1 }
}
