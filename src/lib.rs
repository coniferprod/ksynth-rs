//! # ksynth
//!
//! Patch manipulation helpers for Kawai digital synths.

pub mod k5000;
pub mod k4;

use std::fmt;

/// Error type for parsing data from MIDI System Exclusive bytes.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ParseError {
    InvalidLength(u32, u32),  // actual, expected
    InvalidChecksum(u8, u8),  // actual, expected
    InvalidData(u32),  // offset in data
    Unidentified,  // can't identify this kind
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ParseError::InvalidLength(actual, expected) => format!("Got {} bytes of data, expected {} bytes.", actual, expected),
            ParseError::InvalidChecksum(actual, expected) => format!("Computed checksum was {}H, expected {}H.", actual, expected),
            ParseError::InvalidData(offset) => format!("Invalid data at offset {}.", offset),
            ParseError::Unidentified => String::from("Unable to identify this System Exclusive file."),
        })
    }
}

/// Parsing and generating MIDI System Exclusive data.
pub trait SystemExclusiveData: Sized {
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError>;
    fn to_bytes(&self) -> Vec<u8>;
    fn data_size(&self) -> usize { 0 }
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
