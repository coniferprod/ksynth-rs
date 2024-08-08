use std::fmt;
use rand::Rng;
use std::ops::RangeInclusive;

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

/// A simple struct for wrapping an `i32` with const generic parameters to limit
/// the range of allowed values.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RangedInteger<const MIN: i32, const MAX: i32> {
    value: i32,
}

impl <const MIN: i32, const MAX: i32> RangedInteger<MIN, MAX> {
    /// Makes a new ranged integer if the value is in the allowed range, otherwise panics.
    pub fn new(value: i32) -> Self {
        let range = Self::range();
        if range.contains(&value) {
            Self { value }
        }
        else {
            panic!("new() expected value in range {}...{}, got {}", range.start(), range.end(), value);
        }
    }

    /// Gets the range of allowed values as an inclusive range,
    /// constructed from the generic parameters.
    pub fn range() -> RangeInclusive<i32> {
        MIN ..= MAX
    }

    /// Gets a random value that is in the range of allowed values.
    pub fn random_value() -> i32 {
        let mut rng = rand::thread_rng();
        let range = Self::range();
        rng.gen_range(*range.start() ..= *range.end())
    }
}

/// Trait for a synth parameter.
trait Parameter {
    fn name(&self) -> String;
    fn minimum_value() -> i32;
    fn maximum_value() -> i32;
    fn default_value() -> i32;
    fn random_value() -> i32;
}

// The following types are all based on `RangedInteger`.
// It would be nice if they could be generated with a macro.
// The macro should generate the type to hold the value, with
// the minimum and maximum value. It should also generate the
// implementation for the Parameter trait.

/// Private generic type for the value stored in a `Volume`.
type VolumeValue = RangedInteger::<0, 127>;

/// Wrapper for volume parameter.
#[derive(Debug, Copy, Clone)]
pub struct Volume {
    value: VolumeValue,  // private field to prevent accidental range violations
}

impl Volume {
    /// Makes a new `Volume` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: VolumeValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Volume {
    fn name(&self) -> String {
        "volume".to_string()
    }

    fn minimum_value() -> i32 {
        *VolumeValue::range().start()
    }

    fn maximum_value() -> i32 {
        *VolumeValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        VolumeValue::random_value()
    }
}

impl Default for Volume {
    fn default() -> Self { Self::new(0) }
}

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

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type BenderPitchValue = RangedInteger::<0, 24>;

/// Wrapper for bender pitch parameter.
#[derive(Debug, Copy, Clone)]
pub struct BenderPitch {
    value: BenderPitchValue,  // private field to prevent accidental range violations
}

impl BenderPitch {
    /// Makes a new `BenderPitch` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: BenderPitchValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for BenderPitch {
    fn name(&self) -> String {
        "benderpitch".to_string()
    }

    fn minimum_value() -> i32 {
        *BenderPitchValue::range().start()
    }

    fn maximum_value() -> i32 {
        *BenderPitchValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        BenderPitchValue::random_value()
    }
}

impl Default for BenderPitch {
    fn default() -> Self { Self::new(0) }
}

impl From<u8> for BenderPitch {
    fn from(value: u8) -> BenderPitch {
        BenderPitch::new(value as i32)
    }
}

impl From<BenderPitch> for u8 {
    fn from(val: BenderPitch) -> Self {
        val.value() as u8 // value can be used as such in SysEx
    }
}

impl fmt::Display for BenderPitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type BenderCutoffValue = RangedInteger::<0, 31>;

/// Wrapper for bender pitch parameter.
#[derive(Debug, Copy, Clone)]
pub struct BenderCutoff {
    value: BenderCutoffValue,  // private field to prevent accidental range violations
}

impl BenderCutoff {
    /// Makes a new `BenderCutoff` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: BenderCutoffValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for BenderCutoff {
    fn name(&self) -> String {
        "bendercutoff".to_string()
    }

    fn minimum_value() -> i32 {
        *BenderCutoffValue::range().start()
    }

    fn maximum_value() -> i32 {
        *BenderCutoffValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        BenderCutoffValue::random_value()
    }
}

impl Default for BenderCutoff {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for BenderCutoff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type EnvelopeTimeValue = RangedInteger::<0, 127>;

/// Wrapper for envelope time parameter.
#[derive(Debug, Copy, Clone)]
pub struct EnvelopeTime {
    value: EnvelopeTimeValue,  // private field to prevent accidental range violations
}

impl EnvelopeTime {
    /// Makes a new `EnvelopeTime` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: EnvelopeTimeValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for EnvelopeTime {
    fn name(&self) -> String {
        "EnvelopeTime".to_string()
    }

    fn minimum_value() -> i32 {
        *EnvelopeTimeValue::range().start()
    }

    fn maximum_value() -> i32 {
        *EnvelopeTimeValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        EnvelopeTimeValue::random_value()
    }
}

impl Default for EnvelopeTime {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for EnvelopeTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type EnvelopeLevelValue = RangedInteger::<-63, 63>;

/// Wrapper for envelope level parameter.
#[derive(Debug, Copy, Clone)]
pub struct EnvelopeLevel {
    value: EnvelopeLevelValue,  // private field to prevent accidental range violations
}

impl EnvelopeLevel {
    /// Makes a new `EnvelopeLevel` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: EnvelopeLevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for EnvelopeLevel {
    fn name(&self) -> String {
        "EnvelopeLevel".to_string()
    }

    fn minimum_value() -> i32 {
        *EnvelopeLevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *EnvelopeLevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        EnvelopeLevelValue::random_value()
    }
}

impl Default for EnvelopeLevel {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for EnvelopeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type EnvelopeRateValue = RangedInteger::<0, 127>;

/// Wrapper for envelope rate parameter.
#[derive(Debug, Copy, Clone)]
pub struct EnvelopeRate {
    value: EnvelopeRateValue,  // private field to prevent accidental range violations
}

impl EnvelopeRate {
    /// Makes a new `EnvelopeRate` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: EnvelopeRateValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for EnvelopeRate {
    fn name(&self) -> String {
        "EnvelopeRate".to_string()
    }

    fn minimum_value() -> i32 {
        *EnvelopeRateValue::range().start()
    }

    fn maximum_value() -> i32 {
        *EnvelopeRateValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        EnvelopeRateValue::random_value()
    }
}

impl Default for EnvelopeRate {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for EnvelopeRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type HarmonicEnvelopeLevelValue = RangedInteger::<0, 63>;

/// Wrapper for harmonic envelope level parameter.
#[derive(Debug, Copy, Clone)]
pub struct HarmonicEnvelopeLevel {
    value: HarmonicEnvelopeLevelValue,  // private field to prevent accidental range violations
}

impl HarmonicEnvelopeLevel {
    /// Makes a new `HarmonicEnvelopeLevel` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: HarmonicEnvelopeLevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for HarmonicEnvelopeLevel {
    fn name(&self) -> String {
        "HarmonicEnvelopeLevel".to_string()
    }

    fn minimum_value() -> i32 {
        *HarmonicEnvelopeLevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *HarmonicEnvelopeLevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        HarmonicEnvelopeLevelValue::random_value()
    }
}

impl Default for HarmonicEnvelopeLevel {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for HarmonicEnvelopeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type BiasValue = RangedInteger::<-63, 63>;

/// Wrapper for bias parameter.
#[derive(Debug, Copy, Clone)]
pub struct Bias {
    value: BiasValue,  // private field to prevent accidental range violations
}

impl Bias {
    /// Makes a new `Bias` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: BiasValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Bias {
    fn name(&self) -> String {
        "Bias".to_string()
    }

    fn minimum_value() -> i32 {
        *BiasValue::range().start()
    }

    fn maximum_value() -> i32 {
        *BiasValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        BiasValue::random_value()
    }
}

impl Default for Bias {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Bias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type ControlTimeValue = RangedInteger::<-63, 63>;

/// Wrapper for control time parameter.
#[derive(Debug, Copy, Clone)]
pub struct ControlTime {
    value: ControlTimeValue,  // private field to prevent accidental range violations
}

impl ControlTime {
    /// Makes a new `ControlTime` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: ControlTimeValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for ControlTime {
    fn name(&self) -> String {
        "ControlTime".to_string()
    }

    fn minimum_value() -> i32 {
        *ControlTimeValue::range().start()
    }

    fn maximum_value() -> i32 {
        *ControlTimeValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        ControlTimeValue::random_value()
    }
}

impl Default for ControlTime {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for ControlTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type EnvelopeDepthValue = RangedInteger::<-63, 63>;

/// Wrapper for envelope depth parameter.
#[derive(Debug, Copy, Clone)]
pub struct EnvelopeDepth {
    value: EnvelopeDepthValue,  // private field to prevent accidental range violations
}

impl EnvelopeDepth {
    /// Makes a new `EnvelopeDepth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: EnvelopeDepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for EnvelopeDepth {
    fn name(&self) -> String {
        "EnvelopeDepth".to_string()
    }

    fn minimum_value() -> i32 {
        *EnvelopeDepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *EnvelopeDepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        EnvelopeDepthValue::random_value()
    }
}

impl Default for EnvelopeDepth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for EnvelopeDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type LFOSpeedValue = RangedInteger::<0, 127>;

/// Wrapper for LFO speed parameter.
#[derive(Debug, Copy, Clone)]
pub struct LFOSpeed {
    value: LFOSpeedValue,  // private field to prevent accidental range violations
}

impl LFOSpeed {
    /// Makes a new `LFOSpeed` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: LFOSpeedValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for LFOSpeed {
    fn name(&self) -> String {
        "LFOSpeed".to_string()
    }

    fn minimum_value() -> i32 {
        *LFOSpeedValue::range().start()
    }

    fn maximum_value() -> i32 {
        *LFOSpeedValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        LFOSpeedValue::random_value()
    }
}

impl Default for LFOSpeed {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for LFOSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type LFODepthValue = RangedInteger::<0, 63>;

/// Wrapper for LFO depth parameter.
#[derive(Debug, Copy, Clone)]
pub struct LFODepth {
    value: LFODepthValue,  // private field to prevent accidental range violations
}

impl LFODepth {
    /// Makes a new `LFODepth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: LFODepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for LFODepth {
    fn name(&self) -> String {
        "LFODepth".to_string()
    }

    fn minimum_value() -> i32 {
        *LFODepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *LFODepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        LFODepthValue::random_value()
    }
}

impl Default for LFODepth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for LFODepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type KeyScalingValue = RangedInteger::<-63, 63>;

/// Wrapper for key scaling parameter.
#[derive(Debug, Copy, Clone)]
pub struct KeyScaling {
    value: KeyScalingValue,  // private field to prevent accidental range violations
}

impl KeyScaling {
    /// Makes a new `KeyScaling` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: KeyScalingValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for KeyScaling {
    fn name(&self) -> String {
        "KeyScaling".to_string()
    }

    fn minimum_value() -> i32 {
        *KeyScalingValue::range().start()
    }

    fn maximum_value() -> i32 {
        *KeyScalingValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        KeyScalingValue::random_value()
    }
}

impl Default for KeyScaling {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for KeyScaling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type EffectParameterValue = RangedInteger::<0, 127>;

/// Wrapper for effect parameter.
#[derive(Debug, Copy, Clone)]
pub struct EffectParameter {
    value: EffectParameterValue,  // private field to prevent accidental range violations
}

impl EffectParameter {
    /// Makes a new `EffectParameter` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: EffectParameterValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for EffectParameter {
    fn name(&self) -> String {
        "EffectParameter".to_string()
    }

    fn minimum_value() -> i32 {
        *EffectParameterValue::range().start()
    }

    fn maximum_value() -> i32 {
        *EffectParameterValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        EffectParameterValue::random_value()
    }
}

impl Default for EffectParameter {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for EffectParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type CutoffValue = RangedInteger::<0, 127>;

/// Wrapper for cutoff parameter.
#[derive(Debug, Copy, Clone)]
pub struct Cutoff {
    value: CutoffValue,  // private field to prevent accidental range violations
}

impl Cutoff {
    /// Makes a new `Cutoff` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: CutoffValue::new(value) }
    }

    /// Gets the value wrapped by the private `RangedInteger` field.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Cutoff {
    fn name(&self) -> String {
        "Cutoff".to_string()
    }

    fn minimum_value() -> i32 {
        *CutoffValue::range().start()
    }

    fn maximum_value() -> i32 {
        *CutoffValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        CutoffValue::random_value()
    }
}

impl Default for Cutoff {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Cutoff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type ResonanceValue = RangedInteger::<0, 31>;

/// Wrapper for resonance parameter.
#[derive(Debug, Copy, Clone)]
pub struct Resonance {
    value: ResonanceValue,  // private field to prevent accidental range violations
}

impl Resonance {
    /// Makes a new `Resonance` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: ResonanceValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Resonance {
    fn name(&self) -> String {
        "Resonance".to_string()
    }

    fn minimum_value() -> i32 {
        *ResonanceValue::range().start()
    }

    fn maximum_value() -> i32 {
        *ResonanceValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        ResonanceValue::random_value()
    }
}

impl Default for Resonance {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Resonance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type LevelValue = RangedInteger::<0, 31>;

/// Wrapper for level parameter.
#[derive(Debug, Copy, Clone)]
pub struct Level {
    value: LevelValue,  // private field to prevent accidental range violations
}

impl Level {
    /// Makes a new `Level` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: LevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Level {
    fn name(&self) -> String {
        "Level".to_string()
    }

    fn minimum_value() -> i32 {
        *LevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *LevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        LevelValue::random_value()
    }
}

impl Default for Level {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type PitchEnvelopeLevelValue = RangedInteger::<-63, 63>;

/// Wrapper for pitch envelope level parameter.
#[derive(Debug, Copy, Clone)]
pub struct PitchEnvelopeLevel {
    value: PitchEnvelopeLevelValue,  // private field to prevent accidental range violations
}

impl PitchEnvelopeLevel {
    /// Makes a new `PitchEnvelopeLevel` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: PitchEnvelopeLevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for PitchEnvelopeLevel {
    fn name(&self) -> String {
        "PitchEnvelopeLevel".to_string()
    }

    fn minimum_value() -> i32 {
        *PitchEnvelopeLevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *PitchEnvelopeLevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        PitchEnvelopeLevelValue::random_value()
    }
}

impl Default for PitchEnvelopeLevel {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for PitchEnvelopeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type PitchEnvelopeTimeValue = RangedInteger::<0, 127>;

/// Wrapper for pitch envelope time parameter.
#[derive(Debug, Copy, Clone)]
pub struct PitchEnvelopeTime {
    value: PitchEnvelopeTimeValue,  // private field to prevent accidental range violations
}

impl PitchEnvelopeTime {
    /// Makes a new `PitchEnvelopeTime` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: PitchEnvelopeTimeValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for PitchEnvelopeTime {
    fn name(&self) -> String {
        "PitchEnvelopeTime".to_string()
    }

    fn minimum_value() -> i32 {
        *PitchEnvelopeTimeValue::range().start()
    }

    fn maximum_value() -> i32 {
        *PitchEnvelopeTimeValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        PitchEnvelopeTimeValue::random_value()
    }
}

impl Default for PitchEnvelopeTime {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for PitchEnvelopeTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type VelocityDepthValue = RangedInteger::<0, 127>;

/// Wrapper for velocity depth parameter.
#[derive(Debug, Copy, Clone)]
pub struct VelocityDepth {
    value: VelocityDepthValue,  // private field to prevent accidental range violations
}

impl VelocityDepth {
    /// Makes a new `VelocityDepth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: VelocityDepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for VelocityDepth {
    fn name(&self) -> String {
        "VelocityDepth".to_string()
    }

    fn minimum_value() -> i32 {
        *VelocityDepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *VelocityDepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        VelocityDepthValue::random_value()
    }
}

impl Default for VelocityDepth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for VelocityDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type VelocityControlLevelValue = RangedInteger::<0, 127>;

/// Wrapper for velocity control level parameter.
#[derive(Debug, Copy, Clone)]
pub struct VelocityControlLevel {
    value: VelocityControlLevelValue,  // private field to prevent accidental range violations
}

impl VelocityControlLevel {
    /// Makes a new `VelocityControlLevel` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: VelocityControlLevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for VelocityControlLevel {
    fn name(&self) -> String {
        "VelocityControlLevel".to_string()
    }

    fn minimum_value() -> i32 {
        *VelocityControlLevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *VelocityControlLevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        VelocityControlLevelValue::random_value()
    }
}

impl Default for VelocityControlLevel {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for VelocityControlLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type PortamentoLevelValue = RangedInteger::<0, 127>;

/// Wrapper for portamento level parameter.
#[derive(Debug, Copy, Clone)]
pub struct PortamentoLevel {
    value: PortamentoLevelValue,  // private field to prevent accidental range violations
}

impl PortamentoLevel {
    /// Makes a new `PortamentoLevel` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: PortamentoLevelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for PortamentoLevel {
    fn name(&self) -> String {
        "PortamentoLevel".to_string()
    }

    fn minimum_value() -> i32 {
        *PortamentoLevelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *PortamentoLevelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        PortamentoLevelValue::random_value()
    }
}

impl Default for PortamentoLevel {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for PortamentoLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type KeyOnDelayValue = RangedInteger::<0, 127>;

/// Wrapper for key on delay parameter.
#[derive(Debug, Copy, Clone)]
pub struct KeyOnDelay {
    value: KeyOnDelayValue,  // private field to prevent accidental range violations
}

impl KeyOnDelay {
    /// Makes a new `KeyOnDelay` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: KeyOnDelayValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for KeyOnDelay {
    fn name(&self) -> String {
        "KeyOnDelay".to_string()
    }

    fn minimum_value() -> i32 {
        *KeyOnDelayValue::range().start()
    }

    fn maximum_value() -> i32 {
        *KeyOnDelayValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        KeyOnDelayValue::random_value()
    }
}

impl Default for KeyOnDelay {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for KeyOnDelay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type VelocitySensitivityValue = RangedInteger::<-63, 63>;

/// Wrapper for velocity sensitivity parameter.
#[derive(Debug, Copy, Clone)]
pub struct VelocitySensitivity {
    value: VelocitySensitivityValue,  // private field to prevent accidental range violations
}

impl VelocitySensitivity {
    /// Makes a new `VelocitySensitivity` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: VelocitySensitivityValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for VelocitySensitivity {
    fn name(&self) -> String {
        "VelocitySensitivity".to_string()
    }

    fn minimum_value() -> i32 {
        *VelocitySensitivityValue::range().start()
    }

    fn maximum_value() -> i32 {
        *VelocitySensitivityValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        VelocitySensitivityValue::random_value()
    }
}

impl Default for VelocitySensitivity {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for VelocitySensitivity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type ControlDepthValue  = RangedInteger::<-63, 63>;

/// Wrapper for control depth parameter.
#[derive(Debug, Copy, Clone)]
pub struct ControlDepth {
    value: ControlDepthValue,  // private field to prevent accidental range violations
}

impl ControlDepth {
    /// Makes a new `ControlDepth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: ControlDepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for ControlDepth {
    fn name(&self) -> String {
        "ControlDepth".to_string()
    }

    fn minimum_value() -> i32 {
        *ControlDepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *ControlDepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        ControlDepthValue::random_value()
    }
}

impl Default for ControlDepth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for ControlDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type DepthValue = RangedInteger::<0, 100>;

/// Wrapper for depth parameter.
#[derive(Debug, Copy, Clone)]
pub struct Depth {
    value: DepthValue,  // private field to prevent accidental range violations
}

impl Depth {
    /// Makes a new `Depth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: DepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Depth {
    fn name(&self) -> String {
        "Depth".to_string()
    }

    fn minimum_value() -> i32 {
        *DepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *DepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        DepthValue::random_value()
    }
}

impl Default for Depth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type PanValue = RangedInteger::<-63, 63>;

/// Wrapper for pan parameter.
#[derive(Debug, Copy, Clone)]
pub struct Pan {
    value: PanValue,  // private field to prevent accidental range violations
}

impl Pan {
    /// Makes a new `Pan` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: PanValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Pan {
    fn name(&self) -> String {
        "Pan".to_string()
    }

    fn minimum_value() -> i32 {
        *PanValue::range().start()
    }

    fn maximum_value() -> i32 {
        *PanValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        PanValue::random_value()
    }
}

impl Default for Pan {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Pan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type KeyScalingToGainValue = RangedInteger::<-63, 63>;

/// Wrapper for key scaling to gain parameter.
#[derive(Debug, Copy, Clone)]
pub struct KeyScalingToGain {
    value: KeyScalingToGainValue,  // private field to prevent accidental range violations
}

impl KeyScalingToGain {
    /// Makes a new `KeyScalingToGain` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: KeyScalingToGainValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for KeyScalingToGain {
    fn name(&self) -> String {
        "KeyScalingToGain".to_string()
    }

    fn minimum_value() -> i32 {
        *KeyScalingToGainValue::range().start()
    }

    fn maximum_value() -> i32 {
        *KeyScalingToGainValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        KeyScalingToGainValue::random_value()
    }
}

impl Default for KeyScalingToGain {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for KeyScalingToGain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type CoarseValue = RangedInteger::<-24, 24>;

/// Wrapper for coarse parameter.
#[derive(Debug, Copy, Clone)]
pub struct Coarse {
    value: CoarseValue,  // private field to prevent accidental range violations
}

impl Coarse {
    /// Makes a new `Coarse` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: CoarseValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Coarse {
    fn name(&self) -> String {
        "Coarse".to_string()
    }

    fn minimum_value() -> i32 {
        *CoarseValue::range().start()
    }

    fn maximum_value() -> i32 {
        *CoarseValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        CoarseValue::random_value()
    }
}

impl Default for Coarse {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Coarse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


type FineValue = RangedInteger::<-63, 63>;

/// Wrapper for fine parameter.
#[derive(Debug, Copy, Clone)]
pub struct Fine {
    value: FineValue,  // private field to prevent accidental range violations
}

impl Fine {
    /// Makes a new `Fine` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: FineValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for Fine {
    fn name(&self) -> String {
        "Fine".to_string()
    }

    fn minimum_value() -> i32 {
        *FineValue::range().start()
    }

    fn maximum_value() -> i32 {
        *FineValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        FineValue::random_value()
    }
}

impl Default for Fine {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for Fine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

type MacroParameterDepthValue = RangedInteger::<-31, 31>;

/// Wrapper for macro parameter depth.
#[derive(Debug, Copy, Clone)]
pub struct MacroParameterDepth {
    value: MacroParameterDepthValue,  // private field to prevent accidental range violations
}

impl MacroParameterDepth {
    /// Makes a new `MacroParameterDepth` initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: MacroParameterDepthValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for MacroParameterDepth {
    fn name(&self) -> String {
        "MacroParameterDepth".to_string()
    }

    fn minimum_value() -> i32 {
        *MacroParameterDepthValue::range().start()
    }

    fn maximum_value() -> i32 {
        *MacroParameterDepthValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        MacroParameterDepthValue::random_value()
    }
}

impl Default for MacroParameterDepth {
    fn default() -> Self { Self::new(0) }
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

impl fmt::Display for MacroParameterDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Generates random value that falls in the range of the type.
pub trait RandomValue {
    type T;
    fn random_value(&self) -> Self::T;
}

use nutype::nutype;

/// Patch name.
#[nutype(
    sanitize(with = |s: String| format!("{:<8}", s)),
    validate(not_empty, len_char_max = 8),
    derive(Debug, PartialEq)
)]
pub struct PatchName(String);


type MIDIChannelValue = RangedInteger::<1, 16>;

/// MIDI channel.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MIDIChannel {
    value: MIDIChannelValue,  // private field to prevent accidental range violations
}

impl MIDIChannel {
    /// Makes a new MIDIChannel initialized with the specified value.
    pub fn new(value: i32) -> Self {
        Self { value: MIDIChannelValue::new(value) }
    }

    /// Gets the wrapped value.
    pub fn value(&self) -> i32 {
        self.value.value
    }
}

impl Parameter for MIDIChannel {
    fn name(&self) -> String {
        "MIDIChannel".to_string()
    }

    fn minimum_value() -> i32 {
        *MIDIChannelValue::range().start()
    }

    fn maximum_value() -> i32 {
        *MIDIChannelValue::range().end()
    }

    fn default_value() -> i32 {
        Self::default().value()
    }

    fn random_value() -> i32 {
        MIDIChannelValue::random_value()
    }
}

impl Default for MIDIChannel {
    fn default() -> Self { Self::new(0) }
}

impl From<u8> for MIDIChannel {
    fn from(value: u8) -> MIDIChannel {
        MIDIChannel::new((value as i32) + 1)  // 0~15 to 1~16
    }
}

impl From<MIDIChannel> for u8 {
    fn from(val: MIDIChannel) -> Self {
        (val.value() as u8) - 1  // 1~16 to 0~15
    }
}

impl fmt::Display for MIDIChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_short_patch_name_is_right_padded() {
        let patch_name = PatchName::try_new("Short");
        assert_eq!(patch_name.unwrap().into_inner(), "Short   ");
    }

    #[test]
    fn test_long_patch_name_is_truncated() {
        assert_eq!(
            PatchName::try_new("WayTooLong"),
            Err(PatchNameError::LenCharMaxViolated)
        );
    }

    #[test]
    fn test_midi_channel_from_byte() {
        let ch = MIDIChannel::from(0x00);
        let value = ch.value();
        assert_eq!(value, 1);  // 0x00 goes in, channel should be 1
    }

    #[test]
    fn test_byte_from_midi_channel() {
        let ch = MIDIChannel::new(16);  // channel 16
        let b: u8 = ch.into();
        assert_eq!(b, 0x0F);
    }

}
