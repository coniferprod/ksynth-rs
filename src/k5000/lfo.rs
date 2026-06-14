//! Data model for the LFO.
//!

use std::fmt;
use std::convert::TryFrom;

use rand::Rng;
use num_enum::TryFromPrimitive;
use serde::{Serialize, Deserialize};

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::{
    ByteValue, DESCRIPTORS,
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Control {
    pub depth: i32, // LFODepth,
    pub key_scaling: i32, // KeyScaling,
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
        let depth_desc = DESCRIPTORS.get(&ByteValue::LFODepth).unwrap();
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScaling).unwrap();

        Ok(Self {
            depth: (depth_desc.incoming)(data[0]),
            key_scaling: (ks_desc.incoming)(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let depth_desc = DESCRIPTORS.get(&ByteValue::LFODepth).unwrap();
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScaling).unwrap();

        vec![
            (depth_desc.outgoing)(self.depth), 
            (ks_desc.outgoing)(self.key_scaling),
        ]
    }

    fn data_size() -> usize { 2 }
}

/// LFO settings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lfo {
    pub waveform: Waveform,
    pub speed: i32, // LFOSpeed,
    pub fade_in_time: i32, // LFOSpeed,
    pub fade_in_to_speed: i32, // LFODepth,
    pub delay_onset: i32, // LFOSpeed,
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
        let speed_desc = DESCRIPTORS.get(&ByteValue::LFOSpeed).unwrap();
        let depth_desc = DESCRIPTORS.get(&ByteValue::LFODepth).unwrap();
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScaling).unwrap();

        Ok(Self {
            waveform: Waveform::try_from(data[0]).unwrap(),
            speed: (speed_desc.incoming)(data[1]),
            fade_in_time: (speed_desc.incoming)(data[2]),
            fade_in_to_speed: (depth_desc.incoming)(data[3]),
            delay_onset: (speed_desc.incoming)(data[4]),

            vibrato: Control::from_bytes(&data[5..7])?,
            /*
            vibrato: Control::from_bytes() {
                depth: (depth_desc.incoming)(data[5]),
                key_scaling: (ks_desc.incoming)(data[6]),
            },
             */

            growl: Control::from_bytes(&data[7..9])?,

            /*
            growl: Control {
                depth: (depth_desc.incoming)(data[7]),
                key_scaling: (ks_desc.incoming)(data[8]),
            },
             */

            tremolo: Control::from_bytes(&data[9..11])?,
            /*
            tremolo: Control {
                depth: (depth_desc.incoming)(data[9]),
                key_scaling: (ks_desc.incoming)(data[10]),
            },
             */
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let speed_desc = DESCRIPTORS.get(&ByteValue::LFOSpeed).unwrap();
        let depth_desc = DESCRIPTORS.get(&ByteValue::LFODepth).unwrap();

        result.extend(vec![
            self.waveform as u8,
            (speed_desc.outgoing)(self.speed),
            (speed_desc.outgoing)(self.delay_onset),
            (speed_desc.outgoing)(self.fade_in_time),
            (depth_desc.outgoing)(self.fade_in_to_speed)
        ]);
        result.extend(self.vibrato.to_bytes());
        result.extend(self.growl.to_bytes());
        result.extend(self.tremolo.to_bytes());

        result
    }

    fn data_size() -> usize { 11 }
}

#[cfg(test)]
mod tests {
    use super::{*};

    use serde::{Deserialize, Serialize};
    use serde_xml_rs::{from_str, to_string};

    #[test]
    fn test_serialize_lfo() {
        let lfo: Lfo = Default::default();
        println!("{}", lfo);
        match to_string(&lfo) {
            Ok(s) => eprintln!("LFO as XML = {}", s),
            Err(e) => eprintln!("error: {}", e),
        }
    }

}
