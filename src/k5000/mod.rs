use std::{alloc::System, fmt};

use rand::Rng;
use nutype::nutype;

use crate::{
    Ranged,
    SystemExclusiveData,
    ParseError,
};

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
pub mod multi;

/// Length of patch name
pub const NAME_LENGTH: usize = 8;

/// Trait for a synth parameter.
trait Parameter {
    fn name(&self) -> String;
    fn minimum_value() -> i32;
    fn maximum_value() -> i32;
    fn default_value() -> i32;
    fn random_value() -> i32;
}

/// Volume (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Volume(i32);
crate::ranged_impl!(Volume, 0, 127, 0);

/// BenderPitch (0...24)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderPitch(i32);
crate::ranged_impl!(BenderPitch, 0, 24, 0);

/// BenderCutoff (0...31)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderCutoff(i32);
crate::ranged_impl!(BenderCutoff, 0, 31, 0);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeTime(i32);
crate::ranged_impl!(EnvelopeTime, 0, 127, 0);

/// EnvelopeLevel (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
crate::ranged_impl!(EnvelopeLevel, -63, 63, 0);

/// EnvelopeRate (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeRate(i32);
crate::ranged_impl!(EnvelopeRate, 0, 127, 0);

/// HarmonicEnvelopeLevel (0...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HarmonicEnvelopeLevel(i32);
crate::ranged_impl!(HarmonicEnvelopeLevel, 0, 63, 0);

/// Bias (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bias(i32);
crate::ranged_impl!(Bias, -63, 63, 0);

/// ControlTime (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ControlTime(i32);
crate::ranged_impl!(ControlTime, -63, 63, 0);

/// EnvelopeDepth (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeDepth(i32);
crate::ranged_impl!(EnvelopeDepth, -63, 63, 0);

/// LFOSpeed (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LFOSpeed(i32);
crate::ranged_impl!(LFOSpeed, 0, 127, 0);

/// LFODepth (0...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LFODepth(i32);
crate::ranged_impl!(LFODepth, 0, 63, 0);

/// KeyScaling (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyScaling(i32);
crate::ranged_impl!(KeyScaling, -63, 63, 0);

/// EffectParameter (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectParameter(i32);
crate::ranged_impl!(EffectParameter, 0, 127, 0);

/// Cutoff (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cutoff(i32);
crate::ranged_impl!(Cutoff, 0, 127, 0);

/// Resonance (0...31)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Resonance(i32);
crate::ranged_impl!(Resonance, 0, 31, 0);

/// Level (0...31)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Level(i32);
crate::ranged_impl!(Level, 0, 31, 0);

/// PitchEnvelopeLevel (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PitchEnvelopeLevel(i32);
crate::ranged_impl!(PitchEnvelopeLevel, -63, 63, 0);

/// PitchEnvelopeTime (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PitchEnvelopeTime(i32);
crate::ranged_impl!(PitchEnvelopeTime, 0, 127, 0);

/// VelocityDepth (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocityDepth(i32);
crate::ranged_impl!(VelocityDepth, 0, 127, 0);

/// VelocityControlLevel (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocityControlLevel(i32);
crate::ranged_impl!(VelocityControlLevel, 0, 127, 0);

/// PortamentoLevel (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PortamentoLevel(i32);
crate::ranged_impl!(PortamentoLevel, 0, 127, 0);

/// KeyOnDelay (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyOnDelay(i32);
crate::ranged_impl!(KeyOnDelay, 0, 127, 0);

/// VelocitySensitivity (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct VelocitySensitivity(i32);
crate::ranged_impl!(VelocitySensitivity, -63, 63, 0);

/// ControlDepth (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ControlDepth(i32);
crate::ranged_impl!(ControlDepth, -63, 63, 0);

/// Depth (0...100)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Depth(i32);
crate::ranged_impl!(Depth, 0, 100, 0);

/// Pan (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pan(i32);
crate::ranged_impl!(Pan, -63, 63, 0);

/// KeyScalingToGain (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyScalingToGain(i32);
crate::ranged_impl!(KeyScalingToGain, -63, 63, 0);

/// Coarse (-24...24)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coarse(i32);
crate::ranged_impl!(Coarse, -24, 24, 0);

/// Fine (-63...63)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Fine(i32);
crate::ranged_impl!(Fine, -63, 63, 0);

/// MacroParameterDepth (-31...31)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MacroParameterDepth(i32);
crate::ranged_impl!(MacroParameterDepth, -31, 31, 0);

impl From<u8> for Volume {
    fn from(value: u8) -> Volume {
        Volume::new(value as i32)
    }
}

impl From<Volume> for u8 {
    fn from(val: Volume) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<BenderPitch> for u8 {
    fn from(val: BenderPitch) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for BenderCutoff {
    fn from(value: u8) -> BenderCutoff {
        BenderCutoff::new(value as i32)
    }
}

impl From<BenderCutoff> for u8 {
    fn from(val: BenderCutoff) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for EnvelopeTime {
    fn from(value: u8) -> EnvelopeTime {
        EnvelopeTime::new(value as i32)
    }
}

impl From<EnvelopeTime> for u8 {
    fn from(val: EnvelopeTime) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for EnvelopeLevel {
    fn from(value: u8) -> EnvelopeLevel {
        EnvelopeLevel::new((value as i32) - 64)
    }
}

impl From<EnvelopeLevel> for u8 {
    fn from(val: EnvelopeLevel) -> Self {
        (val.value() + 64) as u8
    }
}

impl From<u8> for EnvelopeRate {
    fn from(value: u8) -> EnvelopeRate {
        EnvelopeRate::new(value as i32)
    }
}

impl From<EnvelopeRate> for u8 {
    fn from(val: EnvelopeRate) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for HarmonicEnvelopeLevel {
    fn from(value: u8) -> HarmonicEnvelopeLevel {
        HarmonicEnvelopeLevel::new(value as i32)
    }
}

impl From<HarmonicEnvelopeLevel> for u8 {
    fn from(val: HarmonicEnvelopeLevel) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for Bias {
    fn from(value: u8) -> Bias {
        Bias::new(value as i32)
    }
}

impl From<Bias> for u8 {
    fn from(val: Bias) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}
impl From<u8> for ControlTime {
    fn from(value: u8) -> ControlTime {
        ControlTime::new((value as i32) - 64)
    }
}

impl From<ControlTime> for u8 {
    fn from(val: ControlTime) -> Self {
        (val.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

impl From<u8> for EnvelopeDepth {
    fn from(value: u8) -> EnvelopeDepth {
        EnvelopeDepth::new((value as i32) - 64)
    }
}

impl From<EnvelopeDepth> for u8 {
    fn from(val: EnvelopeDepth) -> Self {
        (val.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

impl From<u8> for LFOSpeed {
    fn from(value: u8) -> LFOSpeed {
        LFOSpeed::new(value as i32)
    }
}

impl From<LFOSpeed> for u8 {
    fn from(val: LFOSpeed) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for LFODepth {
    fn from(value: u8) -> LFODepth {
        LFODepth::new(value as i32)
    }
}

impl From<LFODepth> for u8 {
    fn from(val: LFODepth) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for KeyScaling {
    fn from(value: u8) -> KeyScaling {
        KeyScaling::new((value as i32) - 64)
    }
}

impl From<KeyScaling> for u8 {
    fn from(val: KeyScaling) -> Self {
        (val.value() + 64) as u8 // value needs adjustment for SysEx
    }
}

impl From<u8> for EffectParameter {
    fn from(value: u8) -> EffectParameter {
        EffectParameter::new(value as i32)
    }
}

impl From<EffectParameter> for u8 {
    fn from(val: EffectParameter) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for Cutoff {
    fn from(value: u8) -> Cutoff {
        Cutoff::new(value as i32)
    }
}

impl From<Cutoff> for u8 {
    fn from(val: Cutoff) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for Resonance {
    fn from(value: u8) -> Resonance {
        Resonance::new(value as i32)
    }
}

impl From<Resonance> for u8 {
    fn from(val: Resonance) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for Level {
    fn from(value: u8) -> Level {
        Level::new(value as i32)
    }
}

impl From<Level> for u8 {
    fn from(val: Level) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}


impl From<u8> for PitchEnvelopeLevel {
    fn from(value: u8) -> PitchEnvelopeLevel {
        PitchEnvelopeLevel::new((value as i32) - 64)
    }
}

impl From<PitchEnvelopeLevel> for u8 {
    fn from(val: PitchEnvelopeLevel) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for PitchEnvelopeTime {
    fn from(value: u8) -> PitchEnvelopeTime {
        PitchEnvelopeTime::new(value as i32)
    }
}

impl From<PitchEnvelopeTime> for u8 {
    fn from(val: PitchEnvelopeTime) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for VelocityDepth {
    fn from(value: u8) -> VelocityDepth {
        VelocityDepth::new(value as i32)
    }
}

impl From<VelocityDepth> for u8 {
    fn from(val: VelocityDepth) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for VelocityControlLevel {
    fn from(value: u8) -> VelocityControlLevel {
        VelocityControlLevel::new(value as i32)
    }
}

impl From<VelocityControlLevel> for u8 {
    fn from(val: VelocityControlLevel) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for PortamentoLevel {
    fn from(value: u8) -> PortamentoLevel {
        PortamentoLevel::new(value as i32)
    }
}

impl From<PortamentoLevel> for u8 {
    fn from(val: PortamentoLevel) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for KeyOnDelay {
    fn from(value: u8) -> KeyOnDelay {
        KeyOnDelay::new(value as i32)
    }
}

impl From<KeyOnDelay> for u8 {
    fn from(val: KeyOnDelay) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl From<u8> for VelocitySensitivity {
    fn from(value: u8) -> VelocitySensitivity {
        VelocitySensitivity::new((value as i32) - 64)
    }
}

impl From<VelocitySensitivity> for u8 {
    fn from(val: VelocitySensitivity) -> Self {
        (val.value() + 64) as u8
    }
}

impl From<u8> for ControlDepth {
    fn from(value: u8) -> ControlDepth {
        ControlDepth::new((value as i32) - 64)
    }
}

impl From<ControlDepth> for u8 {
    fn from(val: ControlDepth) -> Self {
        (val.value() + 64) as u8
    }
}

impl From<u8> for Depth {
    fn from(value: u8) -> Depth {
        Depth::new(value as i32)
    }
}

impl From<Depth> for u8 {
    fn from(val: Depth) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}


impl From<u8> for Pan {
    fn from(value: u8) -> Pan {
        Pan::new((value as i32) - 64)
    }
}

impl From<Pan> for u8 {
    fn from(val: Pan) -> Self {
        (val.value() + 64) as u8
    }
}
impl From<u8> for KeyScalingToGain {
    fn from(value: u8) -> KeyScalingToGain {
        KeyScalingToGain::new((value as i32) - 64)
    }
}

impl From<KeyScalingToGain> for u8 {
    fn from(val: KeyScalingToGain) -> Self {
        (val.value() + 64) as u8
    }
}

impl From<u8> for Coarse {
    fn from(value: u8) -> Coarse {
        Coarse::new((value as i32) - 64)
    }
}

impl From<Coarse> for u8 {
    fn from(val: Coarse) -> Self {
        (val.value() + 64) as u8
    }
}


impl From<u8> for Fine {
    fn from(value: u8) -> Fine {
        Fine::new((value as i32) - 64)
    }
}

impl From<Fine> for u8 {
    fn from(val: Fine) -> Self {
        (val.value() as u8) + 64
    }
}

impl From<u8> for MacroParameterDepth {
    fn from(value: u8) -> MacroParameterDepth {
        MacroParameterDepth::new((value as i32) - 64)  // (-31)33~(+31)95 (K5000W=64)
    }
}

impl From<MacroParameterDepth> for u8 {
    fn from(val: MacroParameterDepth) -> Self {
        (val.value() as u8) + 64
    }
}

/// Generates random value that falls in the range of the type.
pub trait RandomValue {
    type T;
    fn random_value(&self) -> Self::T;
}

/// Patch name.
/*#[nutype(
    sanitize(with = |s: String| format!("{:<8}", s)),
    validate(not_empty, len_char_max = 8),
    derive(Debug, PartialEq, Clone)
)]*/
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PatchName(String);

impl fmt::Display for PatchName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SystemExclusiveData for PatchName {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != NAME_LENGTH {
            return Err(ParseError::InvalidLength(data.len(), NAME_LENGTH));
        }

        match String::from_utf8(data.to_vec()) {
            Ok(name) => Ok(PatchName(name)),
            Err(e) => Err(ParseError::InvalidData(0, format!("invalid name data, error: {}", e)))
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    fn data_size() -> usize { 8 }
}

#[cfg(test)]
mod tests {
    use super::{*};

}
