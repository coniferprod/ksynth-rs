use std::fmt;
use std::collections::HashMap;

use rand::Rng;
use lazy_static::lazy_static;

//use crate::{Ranged, ranged_impl};

pub mod filter;
pub mod amp;
pub mod osc;
pub mod pitch;
pub mod lfo;
pub mod control;
pub mod source;
pub mod effect;
pub mod single;
//pub mod multi;
pub mod morf;
pub mod harmonic;
pub mod formant;
pub mod addkit;
pub mod wave;
pub mod sysex;

/// Length of patch name
pub const NAME_LENGTH: usize = 8;

/*
/// Volume of patch (0...127, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Volume(i32);
ranged_impl!(Volume, 0, 127, 0);

impl From<u8> for Volume {
    fn from(value: u8) -> Volume {
        Self::new(value as i32)
    }
}

impl From<Volume> for u8 {
    fn from(value: Volume) -> Self {
        value.value() as u8
    }
}

/// Bender pitch (0...24, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderPitch(i32);
ranged_impl!(BenderPitch, 0, 24, 0);

impl From<u8> for BenderPitch {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<BenderPitch> for u8 {
    fn from(value: BenderPitch) -> Self {
        value.value() as u8
    }
}

/// Bender cutoff (0...31, default 0).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct BenderCutoff(i32);
ranged_impl!(BenderCutoff, 0, 31, 0);

impl From<u8> for BenderCutoff {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<BenderCutoff> for u8{
    fn from(value: BenderCutoff) -> Self {
        value.value() as u8
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

impl From<EnvelopeTime> for u8 {
    fn from(value: EnvelopeTime) -> Self {
        value.value() as u8
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
        value.value() as u8
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
        (val.value() + 64) as u8
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
        (val.value() + 64) as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        value.value() as u8
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
        (value.value() + 64) as u8
    }
}
*/

/// Byte-sized values from and to SysEx.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ByteValue {
    Volume, // 0...127, default 0
    BenderPitch, // 0...24, default 0
    BenderCutoff,
    EnvelopeTime,
    EnvelopeLevel,
    EnvelopeRate,
    ControlTime,
    EnvelopeDepth,
    EffectParameter,
    Cutoff,
    Resonance,
    Level,
    PitchEnvelopeLevel,
    PitchEnvelopeTime,
    VelocityDepth,
    VelocityControlLevel,
    PortamentoLevel,
    KeyOnDelay,
    VelocitySensitivity,
    ControlDepth,
    Depth,
    Pan,
    KeyScalingToGain,
    Coarse,
    Fine,
    MacroParameterDepth,
    PatchNumber,
    Transpose,
    KeyScaling,
    AmplifierEnvelopeLevel,
    Bias,
    HarmonicEnvelopeLevel,
    LFOSpeed,
    LFODepth,
}

type IncomingFn = fn(u8) -> i32;
type OutgoingFn = fn(i32) -> u8;

/// Descriptors of byte values.
pub struct ByteValueDescriptor {
    pub name: String,  // XML element name (kebab-cased)
    pub first: i32,
    pub last: i32,
    pub default: i32,
    pub incoming: IncomingFn,
    pub outgoing: OutgoingFn,
}

lazy_static! {
    static ref DESCRIPTORS: HashMap<ByteValue, ByteValueDescriptor> = {
        let mut m = HashMap::new();

        m.insert(ByteValue::Volume,
            ByteValueDescriptor { 
                name: "volume".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::BenderPitch,
            ByteValueDescriptor { 
                name: "bender-pitch".to_string(),
                first: 0,
                last: 24,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::BenderCutoff,
            ByteValueDescriptor { 
                name: "bender-cutoff".to_string(),
                first: 0,
                last: 31,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::EnvelopeTime,
            ByteValueDescriptor { 
                name: "time".to_string(),
                first: 0,
                last: 31,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::EnvelopeLevel,
            ByteValueDescriptor { 
                name: "level".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::EnvelopeRate,
            ByteValueDescriptor { 
                name: "rate".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::ControlTime,
            ByteValueDescriptor { 
                name: "control-time".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::EnvelopeDepth,
            ByteValueDescriptor { 
                name: "envelope-depth".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::EffectParameter,
            ByteValueDescriptor { 
                name: "parameter".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Cutoff,
            ByteValueDescriptor { 
                name: "cutoff".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Resonance,
            ByteValueDescriptor { 
                name: "resonance".to_string(),
                first: 0,
                last: 31,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Level,
            ByteValueDescriptor { 
                name: "level".to_string(),
                first: 0,
                last: 31,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::PitchEnvelopeLevel,
            ByteValueDescriptor { 
                name: "level".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::PitchEnvelopeTime,
            ByteValueDescriptor { 
                name: "time".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::VelocityDepth,
            ByteValueDescriptor { 
                name: "velocity-depth".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::VelocityControlLevel,
            ByteValueDescriptor { 
                name: "velocity-control-level".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::PortamentoLevel,
            ByteValueDescriptor { 
                name: "portamento-level".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::KeyOnDelay,
            ByteValueDescriptor { 
                name: "key-on-delay".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::VelocitySensitivity,
            ByteValueDescriptor { 
                name: "velocity-sensitivity".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::ControlDepth,
            ByteValueDescriptor { 
                name: "control-depth".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::Depth,
            ByteValueDescriptor { 
                name: "depth".to_string(),
                first: 0,
                last: 100,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Pan,
            ByteValueDescriptor { 
                name: "pan".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::KeyScalingToGain,
            ByteValueDescriptor { 
                name: "key-scaling-to-gain".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::Coarse,
            ByteValueDescriptor { 
                name: "coarse".to_string(),
                first: -24,
                last: 24,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::Fine,
            ByteValueDescriptor { 
                name: "fine".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::MacroParameterDepth,
            ByteValueDescriptor { 
                name: "coarse".to_string(),
                first: -31,
                last: 31,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::PatchNumber,
            ByteValueDescriptor { 
                name: "number".to_string(),
                first: 0,
                last: 127,
                default: 0,
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Transpose,
            ByteValueDescriptor { 
                name: "transpose".to_string(),
                first: -24,
                last: 24,
                default: 0,
                incoming: |n| n as i32,  // TODO: what/how?
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::KeyScaling,
            ByteValueDescriptor { 
                name: "key-scaling".to_string(),
                first: -63,
                last: 63,
                default: 0,
                incoming: |n| (n as i32) - 64,
                outgoing: |n| (n + 64) as u8,
            }
        );

        m.insert(ByteValue::AmplifierEnvelopeLevel,
            ByteValueDescriptor { 
                name: "level".to_string(), 
                first: 0, 
                last: 127, 
                default: 0, 
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::Bias,  // FF bias (-63...63, default 0)
            ByteValueDescriptor { 
                name: "bias".to_string(), 
                first: -63, 
                last: 63, 
                default: 0, 
                incoming: |n| n as i32,  // TODO: adjustment?
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::HarmonicEnvelopeLevel,  // Harmonic envelope level (0...127, default 0)
            ByteValueDescriptor { 
                name: "harmonic-envelope-level".to_string(), 
                first: 0, 
                last: 127, 
                default: 0, 
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::LFOSpeed, // LFO speed (0...127, default 0)
            ByteValueDescriptor { 
                name: "speed".to_string(), 
                first: 0, 
                last: 127, 
                default: 0, 
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );

        m.insert(ByteValue::LFODepth, // LFO depth (0...63, default 0)        
            ByteValueDescriptor { 
                name: "depth".to_string(), 
                first: 0, 
                last: 63, 
                default: 0, 
                incoming: |n| n as i32,
                outgoing: |n| n as u8,
            }
        );
        
        m
    };
}

#[cfg(test)]
mod tests {
    use super::{*};
}
