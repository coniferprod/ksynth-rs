use std::fmt;
use rand::Rng;
use num;
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
pub struct SignedLevel(i32);

impl SignedLevel {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(-63, 63)
    }

    pub fn is_clamped() -> bool {
        return true
    }

    pub fn new(value: i32) -> Self {
        let range = SignedLevel::range();
        if range.contains(&value) {
            SignedLevel(value)
        }
        else {
            if Self::is_clamped() {
                SignedLevel(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

// Makes new SignedLevel from a byte as found in the SysEx data.
impl From<u8> for SignedLevel {
    fn from(value: u8) -> SignedLevel {
        SignedLevel::new(value as i32 - 64)
    }
}

impl fmt::Display for SignedLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl RandomValue for SignedLevel {
    type T = i32;

    fn random_value(&self) -> Self::T {
        let mut rng = rand::thread_rng();
        let range = SignedLevel::range();
        rng.gen_range(*range.start() ..= *range.end())
    }
}

/// Unsigned level (0...127).
#[derive(Debug, Clone, Copy)]
pub struct UnsignedLevel(i32);

impl UnsignedLevel {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 127)
    }

    pub fn is_clamped() -> bool {
        return true
    }

    pub fn new(value: i32) -> Self {
        let range = UnsignedLevel::range();
        if range.contains(&value) {
            UnsignedLevel(value)
        }
        else {
            if Self::is_clamped() {
                UnsignedLevel(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }
}

impl From<u8> for UnsignedLevel {
    fn from(value: u8) -> UnsignedLevel {
        UnsignedLevel::new(value.into())
    }
}

impl fmt::Display for UnsignedLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl RandomValue for UnsignedLevel {
    type T = i32;

    fn random_value(&self) -> Self::T {
        let mut rng = rand::thread_rng();
        let range = UnsignedLevel::range();
        rng.gen_range(*range.start() ..= *range.end())
    }
}

/// Positive level (1...127).
#[derive(Debug, Clone, Copy)]
pub struct PositiveLevel(i32);

impl PositiveLevel {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(1, 127)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = PositiveLevel::range();
        if range.contains(&value) {
            PositiveLevel(value)
        }
        else {
            if Self::is_clamped() {
                PositiveLevel(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }
}

impl From<u8> for PositiveLevel {
    fn from(value: u8) -> PositiveLevel {
        PositiveLevel::new(value.into())
    }
}

/// Signed depth (-31...+31) (in SysEx 33...95)
#[derive(Debug, Clone, Copy)]
pub struct SignedDepth(i32);

impl SignedDepth {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(-31, 31)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = SignedDepth::range();
        if range.contains(&value) {
            SignedDepth(value)
        }
        else {
            if Self::is_clamped() {
                SignedDepth(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }
}

impl From<u8> for SignedDepth {
    fn from(value: u8) -> SignedDepth {
        SignedDepth::new(value as i32 - 64)
    }
}

impl fmt::Display for SignedDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Unsigned depth (0~63).
#[derive(Debug, Clone, Copy)]
pub struct UnsignedDepth(i32);

impl UnsignedDepth {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 63)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = UnsignedDepth::range();
        if range.contains(&value) {
            UnsignedDepth(value)
        }
        else {
            if Self::is_clamped() {
                UnsignedDepth(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }
}

impl From<u8> for UnsignedDepth {
    fn from(value: u8) -> UnsignedDepth {
        UnsignedDepth::new(value as i32)
    }
}

impl fmt::Display for UnsignedDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Depth (0~100).
#[derive(Debug, Clone, Copy)]
pub struct Depth(i32);

impl Depth {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 100)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = Depth::range();
        if range.contains(&value) {
            Depth(value)
        }
        else {
            if Self::is_clamped() {
                Depth(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }
}

impl From<u8> for Depth {
    fn from(value: u8) -> Depth {
        Depth::new(value as i32)
    }
}

/// Medium depth (0~31).
#[derive(Debug, Clone, Copy)]
pub struct MediumDepth(i32);

impl MediumDepth {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 31)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = MediumDepth::range();
        if range.contains(&value) {
            MediumDepth(value)
        }
        else {
            if Self::is_clamped() {
                MediumDepth(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }
}

impl From<u8> for MediumDepth {
    fn from(value: u8) -> MediumDepth {
        MediumDepth::new(value as i32)
    }
}

impl fmt::Display for MediumDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Small depth (0~7).
#[derive(Debug, Clone, Copy)]
pub struct SmallDepth(i32);

impl SmallDepth {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 31)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = Self::range();
        if range.contains(&value) {
            SmallDepth(value)
        }
        else {
            if Self::is_clamped() {
                SmallDepth(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }

}

impl From<u8> for SmallDepth {
    fn from(value: u8) -> SmallDepth {
        SmallDepth::new(value.into())
    }
}

impl fmt::Display for SmallDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Coarse tuning (-24...+24) (in SysEx 40...88)
#[derive(Debug, Clone, Copy)]
pub struct Coarse(i32);

impl Coarse {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(-24, 24)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = Self::range();
        if range.contains(&value) {
            Coarse(value)
        }
        else {
            if Self::is_clamped() {
                Coarse(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        (self.0 + 64) as u8
    }
}

impl From<u8> for Coarse {
    fn from(value: u8) -> Coarse {
        Coarse::new(value as i32 - 64)
    }
}

impl fmt::Display for Coarse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Unsigned coarse for bender pitch (0...24)
#[derive(Debug, Clone, Copy)]
pub struct UnsignedCoarse(i32);

impl UnsignedCoarse {
    pub fn range() -> RangeInclusive<i32> {
        RangeInclusive::new(0, 24)
    }

    pub fn is_clamped() -> bool {
        true
    }

    pub fn new(value: i32) -> Self {
        let range = Self::range();
        if range.contains(&value) {
            UnsignedCoarse(value)
        }
        else {
            if Self::is_clamped() {
                UnsignedCoarse(num::clamp(value, *range.start(), *range.end()))
            }
            else {
                panic!("expected value in range {}...{}, got {}", *range.start(), *range.end(), value);
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn as_byte(&self) -> u8 {
        self.0 as u8
    }

}

impl From<u8> for UnsignedCoarse {
    fn from(value: u8) -> UnsignedCoarse {
        UnsignedCoarse::new(value as i32)
    }
}

impl fmt::Display for UnsignedCoarse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

// Define additional type aliases
type MacroParameterDepth = SignedDepth;

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
        assert_eq!(ks.value(), -63);
    }

    #[test]
    fn test_signed_level_as_byte() {
        let ks = SignedLevel::new(-63);
        assert_eq!(ks.as_byte(), 1u8);
    }

    #[test]
    fn test_signed_level_clamped() {
        let ks = SignedLevel::new(96);  // too big for range
        assert_eq!(ks.value(), 63); // should be clamped
    }

    #[test]
    fn test_signed_level_clamped_as_byte() {
        let ks = SignedLevel::new(63);
        assert_eq!(ks.as_byte(), 127u8);
    }
}
