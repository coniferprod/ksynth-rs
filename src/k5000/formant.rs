//! Data model for the formant filter.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;

use crate::{
    SystemExclusiveData, 
    ParseError
};
use crate::k5000::morf::Loop;
use crate::k5000::{
    EnvelopeRate, 
    EnvelopeLevel, 
    EnvelopeDepth, 
    Bias, 
    LFODepth, 
    LFOSpeed
};

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
        EnvelopeSegment {
            rate: EnvelopeRate::new(0),
            level: EnvelopeLevel::new(0),
        }
    }
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(EnvelopeSegment {
            rate: EnvelopeRate::from(data[0]),
            level: EnvelopeLevel::from(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate.into(), self.level.into()]
    }

    fn data_size(&self) -> usize { 2 }
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
        Envelope {
            attack: Default::default(),
            decay1: Default::default(),
            decay2: Default::default(),
            release: Default::default(),
            decay_loop: Default::default(),
            velocity_depth: EnvelopeDepth::new(0),
            ks_depth: EnvelopeDepth::new(0),
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Envelope {
            attack: EnvelopeSegment::from_bytes(&data[..2])?,
            decay1: EnvelopeSegment::from_bytes(&data[2..4])?,
            decay2: EnvelopeSegment::from_bytes(&data[4..6])?,
            release: EnvelopeSegment::from_bytes(&data[6..8])?,
            decay_loop: Loop::try_from(data[8]).unwrap(),
            velocity_depth: EnvelopeDepth::from(data[9]),
            ks_depth: EnvelopeDepth::from(data[10]),
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
                self.velocity_depth.into(),
                self.ks_depth.into()
            ]
        );

        result
    }

    fn data_size(&self) -> usize {
        4 * self.attack.data_size()
        + 3
    }
}

/// Formant filter LFO shape.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum LFOShape {
    #[default]
    Triangle,

    Sawtooth,
    Random,
}

/// Formant filter LFO.
#[derive(Debug)]
pub struct Lfo {
    pub speed: LFOSpeed,
    pub shape: LFOShape,
    pub depth: LFODepth,
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo {
            speed: LFOSpeed::new(0),
            shape: Default::default(),
            depth: LFODepth::new(0),
        }
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Lfo {
            speed: LFOSpeed::from(data[0]),
            shape: LFOShape::try_from(data[1]).unwrap(),
            depth: LFODepth::from(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.speed.into(), self.shape as u8, self.depth.into()]
    }

    fn data_size(&self) -> usize { 3 }
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
        FormantFilter {
            bias: Bias::new(0),
            mode: Default::default(),
            envelope_depth: EnvelopeDepth::new(0),
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
        Ok(FormantFilter {
            bias: Bias::from(data[0]),
            mode: Mode::try_from(data[1]).unwrap(),
            envelope_depth: EnvelopeDepth::from(data[2]),
            envelope: Envelope::from_bytes(&data[3..14])?,
            lfo: Lfo::from_bytes(&data[14..])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(
            vec![
                self.bias.into(),
                self.mode as u8,
                self.envelope_depth.into()
            ]
        );
        result.extend(self.envelope.to_bytes());
        result.extend(self.lfo.to_bytes());

        result
    }

    fn data_size(&self) -> usize {
        3 + self.envelope.data_size() + self.lfo.data_size()
    }
}
