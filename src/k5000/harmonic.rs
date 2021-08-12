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
    pub segment0: EnvelopeSegment,
    pub segment1: EnvelopeSegment,
    pub segment2: EnvelopeSegment,
    pub segment3: EnvelopeSegment,
    pub loop_type: Loop,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            segment0: Default::default(),
            segment1: Default::default(),
            segment2: Default::default(),
            segment3: Default::default(),
            loop_type: Default::default(),
        }
    }
}

impl Envelope {
    pub fn new() -> Self {
        let zero_segment = EnvelopeSegment {
            rate: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            level: RangedValue::from_int(RangeKind::PositiveLevel, 0),
        };

        Envelope {
            segment0: zero_segment.clone(),
            segment1: zero_segment.clone(),
            segment2: zero_segment.clone(),
            segment3: zero_segment.clone(),
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
            segment0: EnvelopeSegment {
                rate: segment0_rate,
                level: segment0_level,
            },
            segment1: EnvelopeSegment {
                rate: segment1_rate,
                level: segment1_level,
            },
            segment2: EnvelopeSegment {
                rate: segment2_rate,
                level: segment2_level,
            },
            segment3: EnvelopeSegment {
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

        result.extend(self.segment0.to_bytes());

        // When emitting segment1 and segment2 data,
        // we need to bake the loop type into the levels.

        let mut segment1_level_byte = self.segment1.level.as_byte();
        let mut segment2_level_byte = self.segment2.level.as_byte();

        match self.loop_type {
            Loop::Loop1 => {
                segment1_level_byte.set_bit(6, true);
                segment2_level_byte.set_bit(6, true);
            },
            Loop::Loop2 => {
                segment1_level_byte.set_bit(6, false);
                segment2_level_byte.set_bit(6, true);
            },
            Loop::Off => {
                segment1_level_byte.set_bit(6, false);
                segment2_level_byte.set_bit(6, false);
            }
        }

        let mut segment1_data = self.segment1.to_bytes();
        segment1_data[1] = segment1_level_byte;
        result.extend(segment1_data);

        let mut segment2_data = self.segment2.to_bytes();
        segment2_data[1] = segment2_level_byte;
        result.extend(segment2_data);

        result.extend(self.segment3.to_bytes());

        result
    }
}
