use std::convert::TryInto;
use std::convert::TryFrom;
use std::fmt;
use crate::SystemExclusiveData;
use num_enum::TryFromPrimitive;

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
        writeln!(f,
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

#[derive(Copy, Clone)]
pub struct Lfo {
    pub shape: Shape,
    pub speed: u8,
    pub delay: u8,
    pub depth: i8,
    pub pressure_depth: i8,
}

impl Lfo {
    pub fn new() -> Lfo {
        Lfo {
            shape: Shape::Triangle,
            speed: 0,
            delay: 0,
            depth: 0,
            pressure_depth: 0,
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
        writeln!(f,
            "shape = {}, speed = {}, delay = {}, depth = {}, prs.depth = {}",
            self.shape, self.speed, self.delay, self.depth, self.pressure_depth
        )
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: Vec<u8>) -> Self {
        Lfo {
            shape: Shape::try_from(data[0] & 0x03).unwrap(),
            speed: data[1] & 0x7f,
            delay: data[2] & 0x7f,
            depth: ((data[3] & 0x7f) as i8) - 50, // 0~100 to ±50
            pressure_depth: ((data[4] & 0x7f) as i8) - 50, // 0~100 to ±50
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed,
            self.delay,
            (self.depth + 50).try_into().unwrap(),
            (self.pressure_depth + 50).try_into().unwrap(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size(&self) -> usize { 5 }
}

#[derive(Copy, Clone)]
pub struct Vibrato {
    pub shape: Shape,
    pub speed: u8,  // 0~100
    pub pressure: i8, // -50~+50
    pub depth: i8, // -50~+50
}

impl Vibrato {
    pub fn new() -> Vibrato {
        Vibrato {
            shape: Shape::Triangle,
            speed: 0,
            pressure: 0,
            depth: 0,
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
        writeln!(f,
            "shape = {}, speed = {}, pressure = {}, depth = {}",
            self.shape, self.speed, self.pressure, self.depth
        )
    }
}

impl SystemExclusiveData for Vibrato {
    fn from_bytes(data: Vec<u8>) -> Self {
        Vibrato {
            shape: Shape::try_from((data[0] >> 4) & 0x03).unwrap(),
            speed: data[1] & 0x7f,
            pressure: ((data[2] & 0x7f) as i8) - 50, // 0~100 to ±50
            depth: ((data[3] & 0x7f) as i8) - 50, // 0~100 to ±50
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.shape as u8,
            self.speed,
            (self.pressure + 50).try_into().unwrap(),
            (self.depth + 50).try_into().unwrap(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size(&self) -> usize { 4 }
}
