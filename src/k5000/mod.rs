use std::fmt;
use rand::Rng;
use std::ops::RangeInclusive;

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
pub mod morf;
pub mod harmonic;
pub mod formant;
pub mod addkit;
pub mod wave;
pub mod sysex;

/// Length of patch name
pub const NAME_LENGTH: usize = 8;

/// Volume of patch (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Volume(i32);
ranged_impl!(Volume, 0, 127, 0);

impl From<u8> for Volume {
    fn from(value: u8) -> Volume {
        Self::new(value as i32)
    }
}

impl From<Volume> for u8{
    fn from(value: Volume) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Bender pitch (0...24, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderPitch(i32);
ranged_impl!(BenderPitch, 0, 24, 0);

impl From<u8> for BenderPitch {
    fn from(value: u8) -> BenderPitch {
        Self::new(value as i32)
    }
}

impl From<BenderPitch> for u8{
    fn from(value: BenderPitch) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Bender cutoff (0...31, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderCutoff(i32);
ranged_impl!(BenderCutoff, 0, 31, 0);

impl From<u8> for BenderCutoff {
    fn from(value: u8) -> BenderCutoff {
        Self::new(value as i32)
    }
}

impl From<BenderCutoff> for u8{
    fn from(value: BenderCutoff) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Envelope time (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
ranged_impl!(EnvelopeTime, 0, 127, 0);

impl From<u8> for EnvelopeTime {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<EnvelopeTime> for u8{
    fn from(value: EnvelopeTime) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Envelope level (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
ranged_impl!(EnvelopeLevel, -63, 63, 0);

impl From<u8> for EnvelopeLevel {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<EnvelopeLevel> for u8{
    fn from(value: EnvelopeLevel) -> Self {
        (value.value() + 64) as u8
    }
}

/// Envelope rate (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeRate(i32);
ranged_impl!(EnvelopeRate, 0, 127, 0);

impl From<u8> for EnvelopeRate {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<EnvelopeRate> for u8 {
    fn from(value: EnvelopeRate) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Control time (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct ControlTime(i32);
ranged_impl!(ControlTime, -63, 63, 0);

impl From<u8> for ControlTime {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<ControlTime> for u8 {
    fn from(val: ControlTime) -> Self {
        (val.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

/// Envelope depth (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeDepth(i32);
ranged_impl!(EnvelopeDepth, -63, 63, 0);

impl From<u8> for EnvelopeDepth {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<EnvelopeDepth> for u8 {
    fn from(val: EnvelopeDepth) -> Self {
        (val.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

/// Effect parameter (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectParameter(i32);
ranged_impl!(EffectParameter, 0, 127, 0);

impl From<u8> for EffectParameter {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<EffectParameter> for u8 {
    fn from(value: EffectParameter) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Cutoff (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cutoff(i32);
ranged_impl!(Cutoff, 0, 127, 0);

impl From<u8> for Cutoff {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Cutoff> for u8{
    fn from(value: Cutoff) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Resonance (0...31, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Resonance(i32);
ranged_impl!(Resonance, 0, 31, 0);

impl From<u8> for Resonance {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Resonance> for u8{
    fn from(value: Resonance) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Level (0...31, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Level(i32);
ranged_impl!(Level, 0, 31, 0);

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Level> for u8{
    fn from(value: Level) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Pitch envelope level (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct PitchEnvelopeLevel(i32);
ranged_impl!(PitchEnvelopeLevel, -63, 63, 0);

impl From<u8> for PitchEnvelopeLevel {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<PitchEnvelopeLevel> for u8 {
    fn from(value: PitchEnvelopeLevel) -> Self {
        (value.value() + 64) as u8
    }
}

/// Pitch envelope time (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct PitchEnvelopeTime(i32);
ranged_impl!(PitchEnvelopeTime, 0, 127, 0);

impl From<u8> for PitchEnvelopeTime {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<PitchEnvelopeTime> for u8{
    fn from(value: PitchEnvelopeTime) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Velocity depth (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocityDepth(i32);
ranged_impl!(VelocityDepth, 0, 127, 0);

impl From<u8> for VelocityDepth {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<VelocityDepth> for u8{
    fn from(value: VelocityDepth) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Velocity control level (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocityControlLevel(i32);
ranged_impl!(VelocityControlLevel, 0, 127, 0);

impl From<u8> for VelocityControlLevel {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<VelocityControlLevel> for u8{
    fn from(value: VelocityControlLevel) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Portamento level (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct PortamentoLevel(i32);
ranged_impl!(PortamentoLevel, 0, 127, 0);

impl From<u8> for PortamentoLevel {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<PortamentoLevel> for u8{
    fn from(value: PortamentoLevel) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Key on delay (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyOnDelay(i32);
ranged_impl!(KeyOnDelay, 0, 127, 0);

impl From<u8> for KeyOnDelay {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<KeyOnDelay> for u8{
    fn from(value: KeyOnDelay) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Velocity sensitivity (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocitySensitivity(i32);
ranged_impl!(VelocitySensitivity, -63, 63, 0);

impl From<u8> for VelocitySensitivity {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<VelocitySensitivity> for u8 {
    fn from(value: VelocitySensitivity) -> Self {
        (value.value() + 64) as u8
    }
}

/// ControlDepth (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct ControlDepth(i32);
ranged_impl!(ControlDepth, -63, 63, 0);

impl From<u8> for ControlDepth {
    fn from(value: u8) -> Self {
        ControlDepth::new((value as i32) - 64)
    }
}

impl From<ControlDepth> for u8 {
    fn from(value: ControlDepth) -> Self {
        (value.value() + 64) as u8
    }
}

/// Depth (0...100, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Depth(i32);
ranged_impl!(Depth, 0, 100, 0);

impl From<u8> for Depth {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Depth> for u8{
    fn from(value: Depth) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Pan (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pan(i32);
ranged_impl!(Pan, -63, 63, 0);

impl From<u8> for Pan {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<Pan> for u8 {
    fn from(value: Pan) -> Self {
        (value.value() + 64) as u8
    }
}

/// KeyScalingToGain (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyScalingToGain(i32);
ranged_impl!(KeyScalingToGain, -63, 63, 0);

impl From<u8> for KeyScalingToGain {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<KeyScalingToGain> for u8 {
    fn from(value: KeyScalingToGain) -> Self {
        (value.value() + 64) as u8
    }
}

/// Coarse (-24...24, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coarse(i32);
ranged_impl!(Coarse, -24, 24, 0);

impl From<u8> for Coarse {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<Coarse> for u8 {
    fn from(value: Coarse) -> Self {
        (value.value() + 64) as u8
    }
}

/// Fine (-63...63, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fine(i32);
ranged_impl!(Fine, -63, 63, 0);

impl From<u8> for Fine {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<Fine> for u8 {
    fn from(value: Fine) -> Self {
        (value.value() as u8) + 64
    }
}

/// Macro parameter depth (-31...31, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct MacroParameterDepth(i32);
ranged_impl!(MacroParameterDepth, -31, 31, 0);

impl From<u8> for MacroParameterDepth {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)  // (-31)33~(+31)95 (K5000W=64)
    }
}

impl From<MacroParameterDepth> for u8 {
    fn from(value: MacroParameterDepth) -> Self {
        (value.value() as u8) + 64
    }
}

/// MIDI note (0...127, default 60).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct MIDINote(i32);
ranged_impl!(MIDINote, 0, 127, 60);

impl From<u8> for MIDINote {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<MIDINote> for u8{
    fn from(value: MIDINote) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Patch number (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct PatchNumber(i32);
ranged_impl!(PatchNumber, 0, 127, 0);

impl From<u8> for PatchNumber {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<PatchNumber> for u8{
    fn from(value: PatchNumber) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Transpose (-24...24, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Transpose(i32);
ranged_impl!(Transpose, -24, 24, 0);

impl From<u8> for Transpose {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Transpose> for u8{
    fn from(value: Transpose) -> Self {
        value.value() as u8    // used as such in SysEx, redefine if necessary
    }
}

/// Key scaling (-63...63, default 0)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyScaling(i32);
ranged_impl!(KeyScaling, -63, 63, 0);

impl From<u8> for KeyScaling {
    fn from(value: u8) -> Self {
        Self::new((value as i32) - 64)
    }
}

impl From<KeyScaling> for u8 {
    fn from(value: KeyScaling) -> Self {
        (value.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

#[cfg(test)]
mod tests {
    use super::{*};
}
