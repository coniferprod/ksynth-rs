use std::fmt;
use rand::RngExt;

use syxpack::{
    SystemExclusiveData,
    ParseError,
    Ranged,
    ranged_impl,
    Encoding,
    parse_or_default,
};

/// Graphic equalizer level (-6...6, default 0).
/// In SysEx storage: one byte, 58(-6)~70(+6).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct GEQLevel(i32);
ranged_impl!(GEQLevel, -6, 6, 0);

impl Encoding for GEQLevel {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8
    }
}
/// Number of GEQ bands.
pub const BAND_COUNT: usize = 7;

/// Graphic equalizer settings.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct GEQ {
    pub levels: [GEQLevel; BAND_COUNT],
}

impl GEQ {
    /// Create a new GEQ with default values.
    pub fn new() -> Self {
        Self {
            levels: [GEQLevel::default(); BAND_COUNT],
        }
    }
}

impl Default for GEQ {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemExclusiveData for GEQ {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {    
        let mut levels = [GEQLevel::default(); BAND_COUNT];
        for i in 0..BAND_COUNT {
            let b = data[i];
            levels[i] = parse_or_default::<GEQLevel>(b);
        }
        Ok(Self { levels })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(BAND_COUNT);
        for i in 0..BAND_COUNT {
            let b = self.levels[i].encode();
            bytes.push(b);
        }
        bytes
    }

    fn data_size() -> usize {
        BAND_COUNT
    }
}