//! Data models for the harmonic levels and envelopes.
//!

use bit::BitIndex;

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::morf::Loop;
use crate::k5000::addkit::HARMONIC_COUNT;
use crate::k5000::{
    EnvelopeRate,
    HarmonicEnvelopeLevel
};

pub type Level = u8;

/// Harmonic levels (soft and loud).
pub struct Levels {
    pub soft: [Level; HARMONIC_COUNT],
    pub loud: [Level; HARMONIC_COUNT],
}

impl Default for Levels {
    fn default() -> Self {
        Levels {
            soft: [0; HARMONIC_COUNT],
            loud: [0; HARMONIC_COUNT],
        }
    }
}

impl SystemExclusiveData for Levels {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset = 0;

        let mut soft: [u8; HARMONIC_COUNT] = [0; HARMONIC_COUNT];
        for i in 0..HARMONIC_COUNT {
            soft[i] = data[offset];
            offset += 1;
        }

        let mut loud: [u8; HARMONIC_COUNT] = [0; HARMONIC_COUNT];
        for i in 0..HARMONIC_COUNT {
            loud[i] = data[offset];
            offset += 1;
        }

        Ok(Levels { soft, loud })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.soft.to_vec());
        result.extend(self.loud.to_vec());
        result
    }

    fn data_size() -> usize {
        2 * HARMONIC_COUNT
    }
}

/// Harmonic envelope segment.
#[derive(Debug, Copy, Clone)]
pub struct EnvelopeSegment {
    pub rate: EnvelopeRate,
    pub level: HarmonicEnvelopeLevel,
}

impl Default for EnvelopeSegment {
    fn default() -> Self {
        EnvelopeSegment {
            rate: EnvelopeRate::new(0),
            level: HarmonicEnvelopeLevel::new(0),
        }
    }
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(EnvelopeSegment {
            rate: EnvelopeRate::from(data[0]),
            level: HarmonicEnvelopeLevel::from(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate.into(), self.level.into()]
    }

    fn data_size() -> usize { 2 }
}

/// Harmonic envelope with four segments and loop type.
#[derive(Debug, Copy, Clone, Default)]
pub struct Envelope {
    pub attack: EnvelopeSegment,
    pub decay1: EnvelopeSegment,
    pub decay2: EnvelopeSegment,
    pub release: EnvelopeSegment,
    pub loop_type: Loop,
}

impl Envelope {
    /// Creates a harmonic envelope with default values.
    pub fn new() -> Self {
        let zero_segment = EnvelopeSegment {
            rate: EnvelopeRate::new(0),
            level: HarmonicEnvelopeLevel::new(0),
        };

        Envelope {
            attack: zero_segment,
            decay1: zero_segment,
            decay2: zero_segment,
            release: zero_segment,
            loop_type: Loop::Off,
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let segment0_rate = EnvelopeRate::from(data[0]);
        let segment0_level = HarmonicEnvelopeLevel::from(data[1] & 0b0011_1111);
        let segment1_rate = EnvelopeRate::from(data[2]);
        let segment1_level = HarmonicEnvelopeLevel::from(data[3] & 0b0011_1111);
        let segment1_level_bit6 = data[3].bit(6);
        let segment2_rate = EnvelopeRate::from(data[4]);
        let mut segment2_level_byte = data[5];
        let segment2_level_bit6 = data[5].bit(6);
        segment2_level_byte.set_bit(6, false);
        let segment2_level = HarmonicEnvelopeLevel::from(segment2_level_byte & 0b0011_1111);
        let segment3_rate = EnvelopeRate::from(data[6]);
        let segment3_level = HarmonicEnvelopeLevel::from(data[7] & 0b0011_1111);

        Ok(Envelope {
            attack: EnvelopeSegment {
                rate: segment0_rate,
                level: segment0_level,
            },
            decay1: EnvelopeSegment {
                rate: segment1_rate,
                level: segment1_level,
            },
            decay2: EnvelopeSegment {
                rate: segment2_rate,
                level: segment2_level,
            },
            release: EnvelopeSegment {
                rate: segment3_rate,
                level: segment3_level,
            },
            loop_type: {
                match (segment1_level_bit6, segment2_level_bit6) {
                    (true, false) => Loop::Off,
                    (true, true) => Loop::Loop1,
                    (false, true) => Loop::Loop2,
                    (false, false) => Loop::Off,
                }
            }
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.attack.to_bytes());

        // When emitting decay1 and decay2 data,
        // we need to bake the loop type into the levels.

        let mut decay1_level_byte: u8 = self.decay1.level.into();
        let mut decay2_level_byte: u8 = self.decay2.level.into();

        match self.loop_type {
            Loop::Loop1 => {
                decay1_level_byte.set_bit(6, true);
                decay2_level_byte.set_bit(6, true);
            },
            Loop::Loop2 => {
                decay1_level_byte.set_bit(6, false);
                decay2_level_byte.set_bit(6, true);
            },
            Loop::Off => {
                decay1_level_byte.set_bit(6, false);
                decay2_level_byte.set_bit(6, false);
            }
        }

        let mut decay1_data = self.decay1.to_bytes();
        decay1_data[1] = decay1_level_byte;
        result.extend(decay1_data);

        let mut decay2_data = self.decay2.to_bytes();
        decay2_data[1] = decay2_level_byte;
        result.extend(decay2_data);

        result.extend(self.release.to_bytes());

        result
    }

    fn data_size() -> usize { 8 }
}
