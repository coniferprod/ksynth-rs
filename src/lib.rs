//! # ksynth
//!
//! Patch manipulation helpers for Kawai digital synths.

pub mod k4;
pub mod k5000;

use std::fmt;

use rand::RngExt;
use syxpack::{
    Ranged,
    ranged_impl,
    Encoding,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ValueError(i32, i32, i32);  // expected low, expected high, actual

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected {}...{}, got {}", self.0, self.1, self.2)
    }
}

impl std::error::Error for ValueError { }

/// MIDI note (0...127)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MIDINote(i32);
ranged_impl!(MIDINote, 0, 127, 60);

impl Encoding for MIDINote {}   // using the default implementations

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
