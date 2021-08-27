//! Data models for the harmonic levels and envelopes.
//!

use bit::BitIndex;
use crate::SystemExclusiveData;
use crate::k5000::morf::Loop;
use crate::k5000::addkit::HARMONIC_COUNT;
use crate::k5000::{RangedValue, RangeKind};

/// Harmonic levels (soft and loud).
pub struct Levels {
    pub soft: [u8; HARMONIC_COUNT],
    pub loud: [u8; HARMONIC_COUNT],
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
    fn from_bytes(data: Vec<u8>) -> Self {
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

        Levels {
            soft: soft,
            loud: loud,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.soft.to_vec());
        result.extend(self.loud.to_vec());
        result
    }
}

/// Harmonic envelope segment.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct EnvelopeSegment {
    pub rate: RangedValue,
    pub level: RangedValue,
}

impl Default for EnvelopeSegment {
    fn default() -> Self {
        EnvelopeSegment {
            rate: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            level: RangedValue::from_int(RangeKind::PositiveLevel, 0),
        }
    }
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: Vec<u8>) -> Self {
        EnvelopeSegment {
            rate: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            level: RangedValue::from_byte(RangeKind::PositiveLevel, data[1]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate.as_byte(), self.level.as_byte()]
    }
}

/// Harmonic envelope with four segments and loop type.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Envelope {
    pub attack: EnvelopeSegment,
    pub decay1: EnvelopeSegment,
    pub decay2: EnvelopeSegment,
    pub release: EnvelopeSegment,
    pub loop_type: Loop,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            attack: Default::default(),
            decay1: Default::default(),
            decay2: Default::default(),
            release: Default::default(),
            loop_type: Default::default(),
        }
    }
}

impl Envelope {
    /// Creates a harmonic envelope with default values.
    pub fn new() -> Self {
        let zero_segment = EnvelopeSegment {
            rate: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            level: RangedValue::from_int(RangeKind::PositiveLevel, 0),
        };

        Envelope {
            attack: zero_segment.clone(),
            decay1: zero_segment.clone(),
            decay2: zero_segment.clone(),
            release: zero_segment.clone(),
            loop_type: Loop::Off,
        }
    }

}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        let segment0_rate = RangedValue::from_byte(RangeKind::PositiveLevel, data[0]);
        let segment0_level = RangedValue::from_byte(RangeKind::PositiveLevel, data[1]);
        let segment1_rate = RangedValue::from_byte(RangeKind::PositiveLevel, data[2]);
        let segment1_level = RangedValue::from_byte(RangeKind::PositiveLevel, data[3]);
        let segment1_level_bit6 = data[3].bit(6);
        let segment2_rate = RangedValue::from_byte(RangeKind::PositiveLevel, data[4]);
        let mut segment2_level_byte = data[5];
        let segment2_level_bit6 = data[5].bit(6);
        segment2_level_byte.set_bit(6, false);
        let segment2_level = RangedValue::from_byte(RangeKind::PositiveLevel, segment2_level_byte);
        let segment3_rate = RangedValue::from_byte(RangeKind::PositiveLevel, data[6]);
        let segment3_level = RangedValue::from_byte(RangeKind::PositiveLevel, data[7]);

        Envelope {
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
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.attack.to_bytes());

        // When emitting decay1 and decay2 data,
        // we need to bake the loop type into the levels.

        let mut decay1_level_byte = self.decay1.level.as_byte();
        let mut decay2_level_byte = self.decay2.level.as_byte();

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
}
