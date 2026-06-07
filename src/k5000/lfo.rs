//! Data model for the LFO.
//!

use std::fmt;
use std::convert::TryFrom;

use rand::Rng;
use num_enum::TryFromPrimitive;

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::{
    KeyScaling
};

use crate::{Ranged, ranged_impl};

/// LFO speed (0...127, default 0)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Speed(i32);
ranged_impl!(Speed, 0, 127, 0);

impl From<u8> for Speed {
    fn from(value: u8) -> Speed {
        Speed::new(value as i32)
    }
}

impl From<Speed> for u8 {
    fn from(value: Speed) -> Self {
        value.value() as u8 // value can be used as such in SysEx
    }
}

/// LFO depth (0...63, default 0)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Depth(i32);
ranged_impl!(Depth, 0, 63, 0);

impl From<u8> for Depth {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<Depth> for u8 {
    fn from(value: Depth) -> Self {
        value.value() as u8 // value can be used as such in SysEx
    }
}

/// LFO waveform type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum Waveform {
    #[default]
    Triangle,

    Square,
    Sawtooth,
    Sine,
    Random,
}

impl fmt::Display for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Waveform::Triangle => String::from("TRI"),
            Waveform::Square => String::from("SQR"),
            Waveform::Sawtooth => String::from("SAW"),
            Waveform::Sine => String::from("SIN"),
            Waveform::Random => String::from("RND"),
        })
    }
}

/// LFO control settings.
#[derive(Debug)]
pub struct Control {
    pub depth: Depth,
    pub key_scaling: KeyScaling,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            depth: Default::default(),
            key_scaling: Default::default(),
        }
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Depth={} KS={}", self.depth, self.key_scaling)
    }
}

impl SystemExclusiveData for Control {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            depth: Depth::from(data[0]),
            key_scaling: KeyScaling::from(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.depth.into(), self.key_scaling.into()]
    }

    fn data_size() -> usize { 2 }
}

/// LFO settings.
#[derive(Debug)]
pub struct Lfo {
    pub waveform: Waveform,
    pub speed: Speed,
    pub fade_in_time: Speed,
    pub fade_in_to_speed: Depth,
    pub delay_onset: Speed,
    pub vibrato: Control,
    pub growl: Control,
    pub tremolo: Control,
}

impl Default for Lfo {
    fn default() -> Self {
        Self {
            waveform: Default::default(),
            speed: Default::default(),
            fade_in_time: Default::default(),
            fade_in_to_speed: Default::default(),
            delay_onset: Default::default(),
            vibrato: Default::default(),
            growl: Default::default(),
            tremolo: Default::default(),
        }
    }
}

impl fmt::Display for Lfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Waveform={} Speed={} Fade in Time={} Fade in to Speed={}\nDelay Onset={}\nVibrato: {}\nGrowl: {}\nTremolo: {}\n",
            self.waveform, self.speed, self.fade_in_time, self.fade_in_to_speed,
            self.delay_onset, self.vibrato, self.growl, self.tremolo
        )
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            waveform: Waveform::try_from(data[0]).unwrap(),
            speed: Speed::from(data[1]),
            fade_in_time: Speed::from(data[2]),
            fade_in_to_speed: Depth::from(data[3]),
            delay_onset: Speed::from(data[4]),
            vibrato: Control {
                depth: Depth::from(data[5]),
                key_scaling: KeyScaling::from(data[6]),
            },
            growl: Control {
                depth: Depth::from(data[7]),
                key_scaling: KeyScaling::from(data[8]),
            },
            tremolo: Control {
                depth: Depth::from(data[9]),
                key_scaling: KeyScaling::from(data[10]),
            },
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(vec![
            self.waveform as u8,
            self.speed.into(),
            self.delay_onset.into(),
            self.fade_in_time.into(),
            self.fade_in_to_speed.into()
        ]);
        result.extend(self.vibrato.to_bytes());
        result.extend(self.growl.to_bytes());
        result.extend(self.tremolo.to_bytes());

        result
    }

    fn data_size() -> usize { 11 }
}
