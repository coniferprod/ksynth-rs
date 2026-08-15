//! Data model for the formant filter.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use rand::RngExt;
use syxpack::{
    SystemExclusiveData,
    ParseError,
    Ranged,
    ranged_impl,
    Encoding,
    parse_or_default,
};

use crate::k5000::morf::Loop;
use crate::k5000::{
    EnvelopeRate,
    EnvelopeLevel,
    EnvelopeDepth,
};
use crate::k5000::lfo::{Depth, Speed};

/// FF bias (-63...63, default 0)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Bias(i32);
ranged_impl!(Bias, -63, 63, 0);

impl Encoding for Bias {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// Formant filter envelope mode.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum Mode {
    #[default]
    Envelope,

    Lfo,
}

/// Envelope segment.
#[derive(Debug)]
pub struct EnvelopeSegment {
    pub rate: EnvelopeRate,  // 0~127
    pub level: EnvelopeLevel, // -63(1)~+63(127)
}

impl Default for EnvelopeSegment {
    fn default() -> Self {
        Self {
            rate: Default::default(),
            level: Default::default(),
        }
    }
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            rate: parse_or_default::<EnvelopeRate>(data[0]),
            level: parse_or_default::<EnvelopeLevel>(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.rate.encode(), 
            self.level.encode()
        ]
    }

    fn data_size() -> usize { 2 }
}

/// Formant filter envelope.
#[derive(Debug)]
pub struct Envelope {
    pub attack: EnvelopeSegment,
    pub decay1: EnvelopeSegment,
    pub decay2: EnvelopeSegment,
    pub release: EnvelopeSegment,
    pub decay_loop: Loop,
    pub velocity_depth: EnvelopeDepth,
    pub ks_depth: EnvelopeDepth,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: Default::default(),
            decay1: Default::default(),
            decay2: Default::default(),
            release: Default::default(),
            decay_loop: Default::default(),
            velocity_depth: Default::default(),
            ks_depth: Default::default(),
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            attack: EnvelopeSegment::from_bytes(&data[..2])?,
            decay1: EnvelopeSegment::from_bytes(&data[2..4])?,
            decay2: EnvelopeSegment::from_bytes(&data[4..6])?,
            release: EnvelopeSegment::from_bytes(&data[6..8])?,
            decay_loop: Loop::try_from(data[8]).unwrap(),
            velocity_depth: parse_or_default::<EnvelopeDepth>(data[9]),
            ks_depth: parse_or_default::<EnvelopeDepth>(data[10]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.attack.to_bytes());
        result.extend(self.decay1.to_bytes());
        result.extend(self.decay2.to_bytes());
        result.extend(self.release.to_bytes());
        result.extend(
            vec![
                self.decay_loop as u8,
                self.velocity_depth.encode(),
                self.ks_depth.encode()
            ]
        );

        result
    }

    fn data_size() -> usize {
        4 * EnvelopeSegment::data_size()
        + 3
    }
}

/// Formant filter LFO shape.
#[derive(
    Debug, Copy, Clone,
    Eq, PartialEq, 
    Default,
    TryFromPrimitive
)]
#[repr(u8)]
pub enum Shape {
    #[default]
    Triangle,

    Sawtooth,
    Random,
}

/// Formant filter LFO.
#[derive(Debug)]
pub struct Lfo {
    pub speed: Speed,
    pub shape: Shape,
    pub depth: Depth,
}

impl Default for Lfo {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            shape: Default::default(),
            depth: Default::default(),
        }
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            speed: parse_or_default::<Speed>(data[0]),
            shape: Shape::try_from(data[1]).unwrap(),
            depth: parse_or_default::<Depth>(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.speed.encode(), 
            self.shape as u8, 
            self.depth.encode()
        ]
    }

    fn data_size() -> usize { 3 }
}

/// Formant filter settings.
pub struct FormantFilter {
    pub bias: Bias,
    pub mode: Mode,
    pub envelope_depth: EnvelopeDepth,
    pub envelope: Envelope,
    pub lfo: Lfo,
}

impl Default for FormantFilter {
    fn default() -> Self {
        Self {
            bias: Default::default(),
            mode: Default::default(),
            envelope_depth: Default::default(),
            envelope: Default::default(),
            lfo: Default::default(),
        }
    }
}

impl fmt::Display for FormantFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bias={} Mode={:?} Env.Depth={} Envelope={:?} LFO={:?}",
            self.bias, self.mode, self.envelope_depth,
            self.envelope, self.lfo)
    }
}

impl SystemExclusiveData for FormantFilter {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            bias: parse_or_default::<Bias>(data[0]),
            mode: Mode::try_from(data[1]).unwrap(),
            envelope_depth: parse_or_default::<EnvelopeDepth>(data[2]),
            envelope: Envelope::from_bytes(&data[3..14])?,
            lfo: Lfo::from_bytes(&data[14..])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(
            vec![
                self.bias.encode(),
                self.mode as u8,
                self.envelope_depth.encode()
            ]
        );
        result.extend(self.envelope.to_bytes());
        result.extend(self.lfo.to_bytes());

        result
    }

    fn data_size() -> usize {
        3 + Envelope::data_size() + Lfo::data_size()
    }
}
