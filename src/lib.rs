//! # ksynth
//!
//! Patch manipulation helpers for Kawai digital synths.

pub mod k5000;
pub mod k4;

/// Parsing and generating MIDI System Exclusive data.
pub trait SystemExclusiveData {
    fn from_bytes(data: Vec<u8>) -> Self;
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

fn every_nth_byte(v: &Vec<u8>, n: usize, start: usize) -> Vec<u8> {
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
