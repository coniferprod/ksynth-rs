use crate::SystemExclusiveData;

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

pub const NAME_LENGTH: usize = 10;  // length of patch name
pub const SOURCE_COUNT: usize = 4;  // number of sources in a single patch
pub const DRUM_NOTE_COUNT: usize = 61; // number of DRUM notes
pub const SUBMIX_COUNT: usize = 8;  // number of submix channels / outputs

fn get_effect_number(b: u8) -> u8 {
    let value = b & 0b00011111;
    // Now we should have a value in the range 0~31.
    // Use range 1~32 when storing the value.
    value + 1
}

pub fn get_note_name(note_number: u8) -> String {
    let notes = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ];
    let octave = (note_number / 12) - 2;
    let name = notes[note_number as usize % 12];

    format!("{}{}", name, octave.to_string())
}

// Domain types based on nutype.
// We use the smallest possible inner type for the wrapped value, FWIW.

use nutype::nutype;

// Use for DCA/DCF attack, decay and release
#[nutype(validate(min = 0, max = 100))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EnvelopeTime(u8);

// Used for DCA/DCF sustain
type EnvelopeLevel = EnvelopeTime;

// Level from 0 to 100
#[nutype(validate(min = 0, max = 100))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Level(u8);

// Used for DCA/DCF modulation values
#[nutype(validate(min = -50, max = 50))]
#[derive(Copy, Clone)]
pub struct ModulationDepth(i8);  // note: signed inner type

// MIDI channel
#[nutype(validate(min = 1, max = 16))]
#[derive(Copy, Clone)]
pub struct Channel(u8);

// Used for drum source 1 and 2 decay
#[nutype(validate(min = 1, max = 100))]
#[derive(Copy, Clone)]
pub struct Decay(u8);

#[nutype(validate(min = -7, max = 7))]
#[derive(Copy, Clone)]
pub struct SmallEffectParameter(i8);

#[nutype(validate(min = 0, max = 31))]
#[derive(Copy, Clone)]
pub struct BigEffectParameter(u8);

// Use for DCF sustain
#[nutype(validate(min = -50, max = 50))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FilterEnvelopeLevel(i8);

// Filter cutoff from 0 to 100
#[nutype(validate(min = 0, max = 100))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cutoff(u8);

// Filter resonance from 0 to 7
#[nutype(validate(min = 0, max = 100))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Resonance(u8);

// Effect number 1 to 32
#[nutype(validate(min = 1, max = 32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EffectNumber(u8);

impl SystemExclusiveData for EffectNumber {
    fn from_bytes(data: Vec<u8>) -> Self {
        Self::new(data[0] + 1).unwrap()  // adjust 0~31 to 1~32
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.into_inner() - 1]
    }

    fn data_size(&self) -> usize { 1 }
}

// Velocity or Key Scaling curve 1~8
#[nutype(validate(min = 1, max = 8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Curve(u8);

// DCO coarse tuning
#[nutype(validate(min = -24, max = 24))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coarse(i8);

// DCO fine tuning
#[nutype(validate(min = -50, max = 50))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fine(i8);

// Wave number 1 to 256
#[nutype(validate(min = 1, max = 256))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WaveNumber(u16);

// MIDI channel 1...16
#[nutype(validate(min = 1, max = 16))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MIDIChannel(u8);

impl SystemExclusiveData for MIDIChannel {
    fn from_bytes(data: Vec<u8>) -> Self {
        Self::new(data[0] + 1).unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.into_inner() - 1]
    }

    fn data_size(&self) -> usize { 1 }
}

// Patch number 0...63 (can be converted to A-1...D-16)
#[nutype(validate(min = 0, max = 63))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PatchNumber(u8);

#[nutype(validate(min = -24, max = 24))]  // +-24 (in SysEx 0~48)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transpose(i8);

impl SystemExclusiveData for Transpose {
    fn from_bytes(data: Vec<u8>) -> Self {
        Self::new(data[0] as i8 - 24).unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self.into_inner() + 24) as u8]
    }

    fn data_size(&self) -> usize { 1 }
}
