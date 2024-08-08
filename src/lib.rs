//! # ksynth
//!
//! Patch manipulation helpers for Kawai digital synths.

pub mod k5000;
pub mod k4;

use std::fmt;

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MIDIChannel(i32);

impl MIDIChannel {
    pub fn try_new(value: i32) -> Result<Self, ValueError> {
        if let 1..=16 = value {
            Ok(Self(value))
        } else {
            Err(ValueError(1, 16, value))
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl SystemExclusiveData for MIDIChannel {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() < 1 {
            Err(ParseError::InvalidLength(data.len(), 1))
        } else {
            match MIDIChannel::try_new(data[0].into()) {
                Ok(ch) => Ok(ch),
                Err(e) => Err(ParseError::InvalidData(0, format!("invalid value {}", e)))
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.0 as u8 - 1]  // bring into range 0...15 for SysEx
    }

    fn data_size() -> usize { 1 }
}

use nutype::nutype;

/// MIDI note (0...127)
#[nutype(
    validate(greater_or_equal = 0, less_or_equal = 127),
    derive(Debug, Copy, Clone, PartialEq, Eq)
)]
pub struct MIDINote(u8);


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

    #[test]
    fn test_channel() {
        let ch = MIDIChannel::try_new(1);
        assert!(ch.is_ok());
        //let value = ch.value();
        //assert_eq!(value, 1);  // 0x00 goes in, channel should be 1
    }

    /*
    #[test]
    fn test_byte_from_midi_channel() {
        let ch = MIDIChannel::try_new(16);  // channel 16
        let b: u8 = ch.value();
        assert_eq!(b, 0x0F);
    }
     */
}
