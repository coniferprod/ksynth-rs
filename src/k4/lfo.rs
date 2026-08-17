//! Data model for LFO.
//!

use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use syxpack::{
    Ranged,
    SystemExclusiveData,
    ParseError,
    parse_or_default,
    Encoding,
};

use crate::k4::{
    Level,
    ModulationDepth
};

/// LFO shape.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Shape {
    Triangle,
    Sawtooth,
    Square,
    Random,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}",
            match self {
                Shape::Triangle => "TRI",
                Shape::Sawtooth => "SAW",
                Shape::Square => "SQR",
                Shape::Random => "RND",
            }
        )
    }
}

/// LFO.
#[derive(Copy, Clone)]
pub struct Lfo {
    pub shape: Shape,
    pub speed: Level,  // 0~100
    pub delay: Level,  // 0~100
    pub depth: ModulationDepth,
    pub pressure_depth: ModulationDepth,
}

impl Lfo {
    pub fn new() -> Lfo {
        Lfo {
            shape: Shape::Triangle,
            speed: Level::new(0),
            delay: Level::new(0),
            depth: ModulationDepth::new(0),
            pressure_depth: ModulationDepth::new(0),
        }
    }
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo::new()
    }
}

impl fmt::Display for Lfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "shape = {}, speed = {}, delay = {}, depth = {}, prs.depth = {}",
            self.shape,
            self.speed,
            self.delay,
            self.depth,
            self.pressure_depth
        )
    }
}

impl SystemExclusiveData for Lfo {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Lfo {
            shape: Shape::try_from(data[0] & 0x03).unwrap(),
            speed: parse_or_default::<Level>(data[1] & 0x7f),
            delay: parse_or_default::<Level>(data[2] & 0x7f),
            depth: parse_or_default::<ModulationDepth>(data[3] & 0x7f), // 0~100 to ±50
            pressure_depth: parse_or_default::<ModulationDepth>(data[4] & 0x7f), // 0~100 to ±50
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed.encode(),
            self.delay.encode(),
            self.depth.encode(),
            self.pressure_depth.encode(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size() -> usize { 5 }
}

/// Vibrato settings.
#[derive(Copy, Clone)]
pub struct Vibrato {
    pub shape: Shape,
    pub speed: Level,  // 0~100
    pub pressure: ModulationDepth, // -50~+50
    pub depth: ModulationDepth, // -50~+50
}

impl Vibrato {
    pub fn new() -> Vibrato {
        Vibrato {
            shape: Shape::Triangle,
            speed: Level::new(0),
            pressure: ModulationDepth::new(0),
            depth: ModulationDepth::new(0),
        }
    }
}

impl Default for Vibrato {
    fn default() -> Self {
        Vibrato::new()
    }
}

impl fmt::Display for Vibrato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "shape = {}, speed = {}, pressure = {}, depth = {}",
            self.shape,
            self.speed,
            self.pressure,
            self.depth
        )
    }
}

impl SystemExclusiveData for Vibrato {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Vibrato {
            shape: Shape::try_from((data[0] >> 4) & 0x03).unwrap(),
            speed: parse_or_default::<Level>(data[1] & 0x7f),
            pressure: parse_or_default::<ModulationDepth>(data[2] & 0x7f), // 0~100 to ±50
            depth: parse_or_default::<ModulationDepth>(data[3] & 0x7f), // 0~100 to ±50
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed.encode(),
            self.pressure.encode(),
            self.depth.encode(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size() -> usize { 4 }
}
