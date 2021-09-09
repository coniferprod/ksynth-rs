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
pub mod wave;
pub mod sysex;

// Simple wrapper for an inclusive range of Ord types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Wrapper<T> where T: Ord {
    start: T,
    end: T,
}

/// Generates random value that falls in the range of the type.
pub trait RandomValue {
    type T;
    fn random_value(&self) -> Self::T;
}

// Experiment a little with the newtype pattern.
// A newtype is a special case of a tuple struct,
// with just one field.

/// Signed level (-63...+63)
#[derive(Debug, Clone, Copy)]
pub struct SignedLevel(i8);

impl SignedLevel {
    fn range() -> Wrapper<i8> {
        Wrapper { start: -63, end: 63 }
    }

    pub fn new(value: i8) -> SignedLevel {
        let range = SignedLevel::range();
        SignedLevel(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }

    pub fn value(&self) -> i8 {
        self.0
    }
}

impl From<i8> for SignedLevel {
    fn from(value: i8) -> SignedLevel {
        SignedLevel::new(value)
    }
}

impl From<u8> for SignedLevel {
    fn from(value: u8) -> SignedLevel {
        SignedLevel::new(value as i8 - 64)
    }
}

impl From<i32> for SignedLevel {
    fn from(value: i32) -> SignedLevel {
        SignedLevel::new(value as i8)
    }
}

impl fmt::Display for SignedLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl RandomValue for SignedLevel {
    type T = i8;

    fn random_value(&self) -> Self::T {
        let mut rng = rand::thread_rng();
        let range = SignedLevel::range();
        rng.gen_range(range.start..=range.end)
    }
}

/// Unsigned level (0...127).
#[derive(Debug, Clone, Copy)]
pub struct UnsignedLevel(u8);

impl UnsignedLevel {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 127 }
    }

    pub fn new(value: u8) -> UnsignedLevel {
        let range = UnsignedLevel::range();
        UnsignedLevel(num::clamp(value, range.start, range.end))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for UnsignedLevel {
    fn from(value: u8) -> UnsignedLevel {
        UnsignedLevel::new(value)
    }
}

impl fmt::Display for UnsignedLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Positive level (1...127).
#[derive(Debug, Clone, Copy)]
pub struct PositiveLevel(u8);

impl PositiveLevel {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 1, end: 127 }
    }

    pub fn new(value: u8) -> PositiveLevel {
        let range = PositiveLevel::range();
        PositiveLevel(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PositiveLevel {
    fn from(value: u8) -> PositiveLevel {
        PositiveLevel::new(value)
    }
}

/// Signed depth (-31...+31) (in SysEx 33...95)
#[derive(Debug, Clone, Copy)]
pub struct SignedDepth(i8);

impl SignedDepth {
    fn range() -> Wrapper<i8> {
        Wrapper { start: -31, end: 31 }
    }

    pub fn new(value: i8) -> SignedDepth {
        let range = SignedDepth::range();
        SignedDepth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }
}

impl From<i8> for SignedDepth {
    fn from(value: i8) -> SignedDepth {
        SignedDepth::new(value)
    }
}

impl From<u8> for SignedDepth {
    fn from(value: u8) -> SignedDepth {
        SignedDepth::new(value as i8 - 64)
    }
}

/// Unsigned depth (0~63).
#[derive(Debug, Clone, Copy)]
pub struct UnsignedDepth(u8);

impl UnsignedDepth {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 63 }
    }

    pub fn new(value: u8) -> UnsignedDepth {
        let range = UnsignedDepth::range();
        UnsignedDepth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for UnsignedDepth {
    fn from(value: u8) -> UnsignedDepth {
        UnsignedDepth::new(value)
    }
}

impl fmt::Display for UnsignedDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Depth (0~100).
#[derive(Debug, Clone, Copy)]
pub struct Depth(u8);

impl Depth {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 100 }
    }

    pub fn new(value: u8) -> Depth {
        let range = Depth::range();
        Depth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Depth {
    fn from(value: u8) -> Depth {
        Depth::new(value)
    }
}

/// Macro depth (-31 ... +31).
#[derive(Debug, Clone, Copy)]
pub struct MacroParameterDepth(i8);

impl MacroParameterDepth {
    fn range() -> Wrapper<i8> {
        Wrapper { start: -31, end: 31 }
    }

    pub fn new(value: i8) -> MacroParameterDepth {
        let range = MacroParameterDepth::range();
        MacroParameterDepth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }

    pub fn value(&self) -> i8 {
        self.0
    }
}

impl From<u8> for MacroParameterDepth {
    fn from(value: u8) -> MacroParameterDepth {
        MacroParameterDepth::new(value as i8)
    }
}

impl fmt::Display for MacroParameterDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Medium depth (0~31).
#[derive(Debug, Clone, Copy)]
pub struct MediumDepth(u8);

impl MediumDepth {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 7 }
    }

    pub fn new(value: u8) -> MediumDepth {
        let range = MediumDepth::range();
        MediumDepth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for MediumDepth {
    fn from(value: u8) -> MediumDepth {
        MediumDepth::new(value)
    }
}

/// Small depth (0~7).
#[derive(Debug, Clone, Copy)]
pub struct SmallDepth(u8);

impl SmallDepth {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 7 }
    }

    pub fn new(value: u8) -> SmallDepth {
        let range = SmallDepth::range();
        SmallDepth(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for SmallDepth {
    fn from(value: u8) -> SmallDepth {
        SmallDepth::new(value)
    }
}

impl fmt::Display for SmallDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Coarse tuning (-24...+24) (in SysEx 40...88)
#[derive(Debug, Clone, Copy)]
pub struct Coarse(i8);

impl Coarse {
    fn range() -> Wrapper<i8> {
        Wrapper { start: -24, end: 24 }
    }

    pub fn new(value: i8) -> Coarse {
        let range = Coarse::range();
        Coarse(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }
}

impl From<i8> for Coarse {
    fn from(value: i8) -> Coarse {
        Coarse::new(value)
    }
}

impl From<u8> for Coarse {
    fn from(value: u8) -> Coarse {
        Coarse::new(value as i8 - 64)
    }
}

impl From<i32> for Coarse {
    fn from(value: i32) -> Coarse {
        Coarse::new(value as i8)
    }
}

/// Unsigned coarse for bender pitch (0...24)
#[derive(Debug, Clone, Copy)]
pub struct UnsignedCoarse(u8);

impl UnsignedCoarse {
    fn range() -> Wrapper<u8> {
        Wrapper { start: 0, end: 24 }
    }

    pub fn new(value: u8) -> UnsignedCoarse {
        let range = UnsignedCoarse::range();
        UnsignedCoarse(num::clamp(value, range.start, range.end))
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for UnsignedCoarse {
    fn from(value: u8) -> UnsignedCoarse {
        UnsignedCoarse::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_unsigned_level() {
        let level = UnsignedLevel::from(42);
        assert_eq!(level.as_byte(), 42u8);
    }

    #[test]
    fn test_unsigned_level_clamped() {
        let level = UnsignedLevel::from(192);  // too big for range
        assert_eq!(level.as_byte(), 127u8);  // should be clamped
    }

    #[test]
    fn test_unsigned_depth() {
        let depth = UnsignedDepth::from(42);
        assert_eq!(depth.as_byte(), 42u8);
    }

    #[test]
    fn test_unsigned_depth_clamped() {
        let depth = UnsignedDepth::from(128);  // too big for range
        assert_eq!(depth.as_byte(), 63u8);  // should be clamped
    }

    #[test]
    fn test_signed_level() {
        let ks = SignedLevel::from(1u8);
        assert_eq!(ks.value(), -63i8);
    }

    #[test]
    fn test_signed_level_as_byte() {
        let ks = SignedLevel::from(-63i8);
        assert_eq!(ks.as_byte(), 1u8);
    }

    #[test]
    fn test_signed_level_clamped() {
        let ks = SignedLevel::from(96i8);  // too big for range
        assert_eq!(ks.value(), 63i8); // should be clamped
        assert_eq!(ks.as_byte(), 127u8);
    }

    #[test]
    fn test_signed_level_clamped_as_byte() {
        let ks = SignedLevel::from(63i8);
        assert_eq!(ks.as_byte(), 127u8);
    }
}
