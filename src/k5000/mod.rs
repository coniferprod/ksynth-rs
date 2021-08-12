use std::ops::RangeInclusive;
use std::fmt;
use rand::Rng;
use num;

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
pub mod sysex;

pub trait Ranged {
    const RANGE: RangeInclusive<i16>;

    // Default implementation for random value
    fn random_value() -> i16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(Self::RANGE)
    }

    // Default implementation for random value from a subrange.
    fn random_value_restricted(subrange: RangeInclusive<i16>) -> i16 {
        assert!(subrange.start() >= Self::RANGE.start() && subrange.end() <= Self::RANGE.end());

        let mut rng = rand::thread_rng();
        rng.gen_range(subrange)
    }
}

#[derive(Debug)]
pub struct SignedLevel {
    value: i16,
}

impl Ranged for SignedLevel {
    const RANGE: RangeInclusive<i16> = -63..=63;
}

impl SignedLevel {
    /// Makes a new value from the given byte.
    pub fn from_byte(initial_value: u8) -> SignedLevel {
        SignedLevel {
            value: initial_value as i16 - (Self::RANGE.end() + 1) as i16,
        }
    }

    pub fn from_int(initial_value: i16) -> SignedLevel {
        if Self::RANGE.contains(&initial_value) {
            SignedLevel {
                value: initial_value,
            }
        }
        else {
            SignedLevel {
                value: num::clamp(initial_value, *Self::RANGE.start(), *Self::RANGE.end()),
            }
        }
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary
    pub fn set(&mut self, new_value: i16) {
        self.value = if Self::RANGE.contains(&new_value) {
            new_value
        }
        else {
            num::clamp(new_value, *Self::RANGE.start(), *Self::RANGE.end())
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        (self.value + Self::RANGE.end() + 1) as u8
    }
}

#[derive(Debug)]
pub struct PositiveLevel {
    value: i16,
}

impl Ranged for PositiveLevel {
    const RANGE: RangeInclusive<i16> = 0..=127;
}

impl PositiveLevel {
    /// Makes a new value from the given byte.
    pub fn from_byte(initial_value: u8) -> PositiveLevel {
        PositiveLevel {
            value: initial_value as i16,
        }
    }

    pub fn from_int(initial_value: i16) -> PositiveLevel {
        if Self::RANGE.contains(&initial_value) {
            PositiveLevel {
                value: initial_value,
            }
        }
        else {
            PositiveLevel {
                value: num::clamp(initial_value, *Self::RANGE.start(), *Self::RANGE.end()),
            }
        }
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary
    pub fn set(&mut self, new_value: i16) {
        self.value = if Self::RANGE.contains(&new_value) {
            new_value
        }
        else {
            num::clamp(new_value, *Self::RANGE.start(), *Self::RANGE.end())
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        self.value as u8
    }
}

#[derive(Debug)]
pub struct UnsignedLevel {
    value: i16,
}

impl Ranged for UnsignedLevel {
    const RANGE: RangeInclusive<i16> = 0..=63;
}

impl UnsignedLevel {
    /// Makes a new value from the given byte.
    pub fn from_byte(initial_value: u8) -> UnsignedLevel {
        UnsignedLevel {
            value: initial_value as i16,
        }
    }

    pub fn from_int(initial_value: i16) -> UnsignedLevel {
        if Self::RANGE.contains(&initial_value) {
            UnsignedLevel {
                value: initial_value,
            }
        }
        else {
            UnsignedLevel {
                value: num::clamp(initial_value, *Self::RANGE.start(), *Self::RANGE.end()),
            }
        }
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary
    pub fn set(&mut self, new_value: i16) {
        self.value = if Self::RANGE.contains(&new_value) {
            new_value
        }
        else {
            num::clamp(new_value, *Self::RANGE.start(), *Self::RANGE.end())
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        (self.value + Self::RANGE.end() + 1) as u8
    }
}

#[derive(Debug)]
pub struct VelocityCurve {
    value: i16,
}

impl Ranged for VelocityCurve {
    const RANGE: RangeInclusive<i16> = 1..=12;
}

impl VelocityCurve {
    /// Makes a new value from the given byte.
    pub fn from_byte(initial_value: u8) -> VelocityCurve {
        VelocityCurve {
            value: initial_value as i16,
        }
    }

    pub fn from_int(initial_value: i16) -> VelocityCurve {
        if Self::RANGE.contains(&initial_value) {
            VelocityCurve {
                value: initial_value,
            }
        }
        else {
            VelocityCurve {
                value: num::clamp(initial_value, *Self::RANGE.start(), *Self::RANGE.end()),
            }
        }
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary
    pub fn set(&mut self, new_value: i16) {
        self.value = if Self::RANGE.contains(&new_value) {
            new_value
        }
        else {
            num::clamp(new_value, *Self::RANGE.start(), *Self::RANGE.end())
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        (self.value + Self::RANGE.end() + 1) as u8
    }
}

#[derive(Debug)]
pub struct MacroDepth {
    value: i16,
}

impl Ranged for MacroDepth {
    const RANGE: RangeInclusive<i16> = 1..=12;
}

impl MacroDepth {
    /// Makes a new value from the given byte.
    pub fn from_byte(initial_value: u8) -> MacroDepth {
        MacroDepth {
            value: initial_value as i16,
        }
    }

    pub fn from_int(initial_value: i16) -> MacroDepth {
        if Self::RANGE.contains(&initial_value) {
            MacroDepth {
                value: initial_value,
            }
        }
        else {
            MacroDepth {
                value: num::clamp(initial_value, *Self::RANGE.start(), *Self::RANGE.end()),
            }
        }
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary
    pub fn set(&mut self, new_value: i16) {
        self.value = if Self::RANGE.contains(&new_value) {
            new_value
        }
        else {
            num::clamp(new_value, *Self::RANGE.start(), *Self::RANGE.end())
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        (self.value + Self::RANGE.end() + 1) as u8
    }
}

pub type IntegerRange = RangeInclusive<i16>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RangeKind {
    SignedLevel,
    UnsignedLevel,
    PositiveLevel,
    VelocityCurve,
    MacroDepth,
    EffectDepth,
    FilterLevel,
    FilterResonance,
    CoarseTuning,
    BenderPitch,
    BenderCutoff,
}

// Rust ranges are not Copy because reasons (see https://github.com/rust-lang/rfcs/issues/2848),
// so let's use a wrapper:
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct RangeInclusiveWrapper {
    start: i16,
    end: i16,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RangedValue {
    kind: RangeKind,
    value: i16,
    range: RangeInclusiveWrapper, // used to be range: RangeInclusive<i16>,
}

impl Default for RangedValue {
    fn default() -> Self {
        RangedValue {
            kind: RangeKind::PositiveLevel,
            value: 0,
            range: RangedValue::make_range(RangeKind::PositiveLevel),
        }
    }
}

impl RangedValue {
    fn make_range(kind: RangeKind) -> RangeInclusiveWrapper {
        // It would have been so nice just to say `-63..=63`...
        match kind {
            RangeKind::SignedLevel => RangeInclusiveWrapper { start: -63, end: 63 },
            RangeKind::UnsignedLevel => RangeInclusiveWrapper { start: 0, end: 63 },
            RangeKind::PositiveLevel => RangeInclusiveWrapper { start: 0, end: 127 },
            RangeKind::VelocityCurve => RangeInclusiveWrapper { start: 1, end: 12 },
            RangeKind::MacroDepth => RangeInclusiveWrapper { start: -31, end: 31 },
            RangeKind::EffectDepth => RangeInclusiveWrapper { start: 0, end: 100 },
            RangeKind::FilterLevel => RangeInclusiveWrapper { start: 0, end: 7 },
            RangeKind::FilterResonance => RangeInclusiveWrapper { start: 0, end: 7 },
            RangeKind::CoarseTuning => RangeInclusiveWrapper { start: -24, end: 24 },
            RangeKind::BenderPitch => RangeInclusiveWrapper { start: 0, end: 24 },
            RangeKind::BenderCutoff => RangeInclusiveWrapper { start: 0, end: 31 },
        }
    }

    /// Makes a new value from the given byte.
    pub fn from_byte(kind: RangeKind, initial_value: u8) -> RangedValue {
        let range = RangedValue::make_range(kind);

        // If this were a regular RangeInclusive, these would be calls to start() and end().

        let value = if range.start < 0 {  // need to adjust
            initial_value as i16 - (range.end + 1) as i16
        }
        else {
            initial_value as i16
        };

        RangedValue {
            kind,
            range,
            value,
        }
    }

    /// Makes a new ranged value from the given integer.
    pub fn from_int(kind: RangeKind, initial_value: i16) -> RangedValue {
        let range = RangedValue::make_range(kind);


        let value = if initial_value >= range.start && initial_value <= range.end {
            initial_value
        }
        else {
            num::clamp(initial_value, range.start, range.end)
        };

        RangedValue {
            kind,
            range,
            value
        }
    }

    /// Gets the range of this value.
    pub fn get_range(&self) -> RangeInclusive<i16> {
        // Make a new normal range from our wrapper
        self.range.start..=self.range.end
    }

    // Gets the range kind of this value.
    pub fn get_kind(&self) -> RangeKind {
        self.kind
    }

    /// Gets the current value.
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the current value, clamping it if necessary.
    pub fn set(&mut self, new_value: i16) {
        self.value = if new_value >= self.range.start && new_value <= self.range.end {
            new_value
        }
        else {
            num::clamp(new_value, self.range.start, self.range.end)
        }
    }

    /// Gets the value as a byte.
    pub fn as_byte(&self) -> u8 {
        if self.range.start < 0 {
            (self.value + self.range.end + 1) as u8
        }
        else {
            self.value as u8
        }
    }

    // Default implementation for random value.
    pub fn random_value(&self) -> i16 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.range.start..=self.range.end)
    }

    // Default implementation for random value from a subrange.
    pub fn random_value_restricted(&self, subrange: IntegerRange) -> i16 {
        assert!(subrange.start() >= &self.range.start && subrange.end() <= &self.range.end);

        let mut rng = rand::thread_rng();
        rng.gen_range(subrange)
    }
}

impl fmt::Display for RangedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Parameter {
    pub name: String,
    pub value: RangedValue,
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_signed_level_from_byte() {
        let level = SignedLevel::from_byte(16u8);
        assert_eq!(level.get(), -48);
    }

    #[test]
    fn test_signed_level_from_int() {
        let level = SignedLevel::from_int(16);
        assert_eq!(level.get(), 16);
    }

    #[test]
    fn test_signed_level_from_int_clamped() {
        let level = SignedLevel::from_int(100);

        // The value should be clamped to the end of the range
        assert_eq!(level.get(), 63);
    }

    #[test]
    fn test_signed_level_as_byte() {
        let level = SignedLevel::from_int(16);
        assert_eq!(level.as_byte(), 80u8);
    }

    #[test]
    fn test_signed_level_set() {
        let mut level = SignedLevel::from_int(16);
        level.set(17);
        assert_eq!(level.get(), 17);
    }

    #[test]
    fn test_signed_level_set_clamped() {
        let mut level = SignedLevel::from_int(16);
        level.set(-100);  // deliberately out of the allowed range

        // The value should be clamped to the start of the range
        assert_eq!(level.get(), -63);
    }

    #[test]
    fn test_ranged_value_from_byte() {
        let level = RangedValue::from_byte(RangeKind::SignedLevel, 16u8);
        assert_eq!(level.get(), -48);
    }

    #[test]
    fn test_ranged_value_from_int() {
        let level = RangedValue::from_int(RangeKind::SignedLevel, 16);
        assert_eq!(level.get(), 16);
    }

    #[test]
    fn test_ranged_value_from_int_clamped() {
        let level = RangedValue::from_int(RangeKind::SignedLevel, 100);

        // The value should be clamped to the end of the range
        assert_eq!(level.get(), 63);
    }

    #[test]
    fn test_ranged_value_as_byte() {
        let level = RangedValue::from_int(RangeKind::SignedLevel, 16);
        assert_eq!(level.as_byte(), 80u8);
    }

    #[test]
    fn test_ranged_value_set() {
        let mut level = RangedValue::from_int(RangeKind::SignedLevel, 16);
        level.set(17);
        assert_eq!(level.get(), 17);
    }

    #[test]
    fn test_ranged_value_set_clamped() {
        let mut level = RangedValue::from_int(RangeKind::SignedLevel, 16);
        level.set(-100);  // deliberately out of the allowed range

        // The value should be clamped to the start of the range
        assert_eq!(level.get(), -63);
    }
}
