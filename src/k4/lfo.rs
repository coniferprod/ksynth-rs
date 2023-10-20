//! Data model for LFO.
//!

use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;

use crate::SystemExclusiveData;
use crate::k4::{Level, ModulationDepth};

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
            speed: Level::new(0).unwrap(),
            delay: Level::new(0).unwrap(),
            depth: ModulationDepth::new(0).unwrap(),
            pressure_depth: ModulationDepth::new(0).unwrap(),
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
            self.speed.into_inner(),
            self.delay.into_inner(),
            self.depth.into_inner(),
            self.pressure_depth.into_inner()
        )
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: Vec<u8>) -> Self {
        Lfo {
            shape: Shape::try_from(data[0] & 0x03).unwrap(),
            speed: Level::new(data[1] & 0x7f).unwrap(),
            delay: Level::new(data[2] & 0x7f).unwrap(),
            depth: ModulationDepth::new(((data[3] & 0x7f) as i8) - 50).unwrap(), // 0~100 to ±50
            pressure_depth: ModulationDepth::new(((data[4] & 0x7f) as i8) - 50).unwrap(), // 0~100 to ±50
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed.into_inner(),
            self.delay.into_inner(),
            (self.depth.into_inner() + 50).try_into().unwrap(),
            (self.pressure_depth.into_inner() + 50).try_into().unwrap(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size(&self) -> usize { 5 }
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
            speed: Level::new(0).unwrap(),
            pressure: ModulationDepth::new(0).unwrap(),
            depth: ModulationDepth::new(0).unwrap(),
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
            self.speed.into_inner(),
            self.pressure.into_inner(),
            self.depth.into_inner()
        )
    }
}

impl SystemExclusiveData for Vibrato {
    fn from_bytes(data: Vec<u8>) -> Self {
        Vibrato {
            shape: Shape::try_from((data[0] >> 4) & 0x03).unwrap(),
            speed: Level::new(data[1] & 0x7f).unwrap(),
            pressure: ModulationDepth::new(((data[2] & 0x7f) as i8) - 50).unwrap(), // 0~100 to ±50
            depth: ModulationDepth::new(((data[3] & 0x7f) as i8) - 50).unwrap(), // 0~100 to ±50
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed.into_inner(),
            (self.pressure.into_inner() + 50).try_into().unwrap(),
            (self.depth.into_inner() + 50).try_into().unwrap(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size(&self) -> usize { 4 }
}
