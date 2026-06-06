//! # ksynth
//!
//! Patch manipulation helpers for Kawai digital synths.

pub mod k5000;
pub mod k4;

use std::fmt;
use rand::Rng;

/// Error type for parsing data from MIDI System Exclusive bytes.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseError {
    InvalidLength(usize, usize),  // actual, expected
    InvalidChecksum(u8, u8),  // actual, expected
    InvalidData(u32, String),  // offset in data, explanation
    Unidentified,  // can't identify this kind
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ParseError::InvalidLength(actual, expected) => format!("Got {} bytes of data, expected {} bytes.", actual, expected),
            ParseError::InvalidChecksum(actual, expected) => format!("Computed checksum was {}H, expected {}H.", actual, expected),
            ParseError::InvalidData(offset, message) => format!("Invalid data at offset {}. Reason: {}", offset, message),
            ParseError::Unidentified => String::from("Unable to identify this System Exclusive file."),
        })
    }
}

impl std::error::Error for ParseError { }

/// Parsing and generating MIDI System Exclusive data.
pub trait SystemExclusiveData: Sized {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError>;
    fn to_bytes(&self) -> Vec<u8>;
    fn data_size() -> usize;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ValueError(i32, i32, i32);  // expected low, expected high, actual

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected {}...{}, got {}", self.0, self.1, self.2)
    }
}

impl std::error::Error for ValueError { }

pub trait Ranged {
    const FIRST: i32;
    const LAST: i32;
    const DEFAULT: i32;

    fn new(value: i32) -> Self;
    fn value(&self) -> i32;
    fn contains(value: i32) -> bool;
    fn random() -> Self;
}

// The `ranged_impl` macro generates an implementation of the `Ranged` trait,
// along with implementations of the `Default` and `Display` traits based on
// the values supplied as parameters (type name, first, last, default).
#[macro_export]
macro_rules! ranged_impl {
    ($typ:ty, $first:expr, $last:expr, $default:expr) => {
        impl Ranged for $typ {
            const FIRST: i32 = $first;
            const LAST: i32 = $last;
            const DEFAULT: i32 = $default;

            fn new(value: i32) -> Self {
                if Self::contains(value) {
                    Self(value)
                }
                else {
                    panic!("{} expected value in range [{}...{}], got {}",
                        stringify!($typ), Self::FIRST, Self::LAST, value);
                }
            }

            fn value(&self) -> i32 { self.0 }

            fn contains(value: i32) -> bool {
                value >= Self::FIRST && value <= Self::LAST
            }

            fn random() -> Self {
                let mut rng = rand::rng();
                Self::new(rng.random_range(Self::FIRST..=Self::LAST))
            }
        }

        impl Default for $typ {
            fn default() -> Self {
                if Self::contains(Self::DEFAULT) {
                    Self::new(Self::DEFAULT)
                } else {
                    panic!("default value {} not in range [{}...{}]",
                        Self::DEFAULT, Self::FIRST, Self::LAST);
                }
            }
        }

        impl fmt::Display for $typ {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

/// MIDI channel (1...16)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MIDIChannel(i32);
crate::ranged_impl!(MIDIChannel, 1, 16, 1);

/// MIDI note (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MIDINote(i32);
crate::ranged_impl!(MIDINote, 0, 127, 60);

impl MIDINote {
    pub fn name(&self) -> String {
        let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ];
        let octave = (self.0 / 12) - 2;
        let name = notes[self.0 as usize % 12];

        format!("{}{}", name, octave)
    }
}

/// Checksum for a patch.
pub trait Checksum {
    fn checksum(&self) -> u8;
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn every_nth_byte(v: &[u8], n: usize, start: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    for (index, _value) in v.iter().enumerate() {
        if index % n == 0 {
            buf.push(v[index + start]);
        }
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_nth_byte() {
        let data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        assert_eq!(every_nth_byte(&data1, 4, 0), vec![1, 5, 9]);

        let data2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        assert_eq!(every_nth_byte(&data2, 4, 1), vec![2, 6, 10]);
    }
}
