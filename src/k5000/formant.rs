//! Data model for the formant filter.
//!

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::k5000::morf::Loop;
use crate::k5000::{RangedValue, RangeKind};

/// Formant filter envelope mode.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    Envelope,
    Lfo,
}

impl Default for Mode {
    fn default() -> Self { Mode::Envelope }
}

/// Envelope segment.
pub struct EnvelopeSegment {
    pub rate: RangedValue,  // 0~127
    pub level: RangedValue, // -63(1)~+63(127)
}

impl Default for EnvelopeSegment {
    fn default() -> Self {
        EnvelopeSegment {
            rate: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            level: RangedValue::from_int(RangeKind::SignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: Vec<u8>) -> Self {
        EnvelopeSegment {
            rate: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            level: RangedValue::from_byte(RangeKind::SignedLevel, data[1]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate.as_byte(), self.level.as_byte()]
    }
}

/// Formant filter envelope.
pub struct Envelope {
    pub attack: EnvelopeSegment,
    pub decay1: EnvelopeSegment,
    pub decay2: EnvelopeSegment,
    pub release: EnvelopeSegment,
    pub decay_loop: Loop,
    pub velocity_depth: RangedValue,
    pub ks_depth: RangedValue,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            attack: Default::default(),
            decay1: Default::default(),
            decay2: Default::default(),
            release: Default::default(),
            decay_loop: Default::default(),
            velocity_depth: RangedValue::from_int(RangeKind::SignedLevel, 0),
            ks_depth: RangedValue::from_int(RangeKind::SignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            attack: EnvelopeSegment::from_bytes(data[..2].to_vec()),
            decay1: EnvelopeSegment::from_bytes(data[2..4].to_vec()),
            decay2: EnvelopeSegment::from_bytes(data[4..6].to_vec()),
            release: EnvelopeSegment::from_bytes(data[6..8].to_vec()),
            decay_loop: Loop::try_from(data[8]).unwrap(),
            velocity_depth: RangedValue::from_byte(RangeKind::SignedLevel, data[9]),
            ks_depth: RangedValue::from_byte(RangeKind::SignedLevel, data[10]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.attack.to_bytes());
        result.extend(self.decay1.to_bytes());
        result.extend(self.decay2.to_bytes());
        result.extend(self.release.to_bytes());
        result.extend(vec![self.decay_loop as u8, self.velocity_depth.as_byte(), self.ks_depth.as_byte()]);

        result
    }
}

/// Formant filter LFO shape.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum LfoShape {
    Triangle,
    Sawtooth,
    Random,
}

impl Default for LfoShape {
    fn default() -> Self { LfoShape::Triangle }
}

/// Formant filter LFO.
pub struct Lfo {
    pub speed: RangedValue,
    pub shape: LfoShape,
    pub depth: RangedValue,
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo {
            speed: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            shape: Default::default(),
            depth: RangedValue::from_int(RangeKind::UnsignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: Vec<u8>) -> Self {
        Lfo {
            speed: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            shape: LfoShape::try_from(data[1]).unwrap(),
            depth: RangedValue::from_byte(RangeKind::UnsignedLevel, data[2]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.speed.as_byte(), self.shape as u8, self.depth.as_byte()]
    }
}

/// Formant filter settings.
pub struct FormantFilter {
    pub bias: RangedValue,
    pub mode: Mode,
    pub envelope_depth: RangedValue,
    pub envelope: Envelope,
    pub lfo: Lfo,
}

impl Default for FormantFilter {
    fn default() -> Self {
        FormantFilter {
            bias: RangedValue::from_int(RangeKind::SignedLevel, 0),
            mode: Default::default(),
            envelope_depth: RangedValue::from_int(RangeKind::SignedLevel, 0),
            envelope: Default::default(),
            lfo: Default::default(),
        }
    }
}

impl SystemExclusiveData for FormantFilter {
    fn from_bytes(data: Vec<u8>) -> Self {
        FormantFilter {
            bias: RangedValue::from_byte(RangeKind::SignedLevel, data[0]),
            mode: Mode::try_from(data[1]).unwrap(),
            envelope_depth: RangedValue::from_byte(RangeKind::SignedLevel, data[2]),
            envelope: Envelope::from_bytes(data[3..14].to_vec()),
            lfo: Lfo::from_bytes(data[14..].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(vec![self.bias.as_byte(), self.mode as u8, self.envelope_depth.as_byte()]);
        result.extend(self.envelope.to_bytes());
        result.extend(self.lfo.to_bytes());

        result
    }
}
