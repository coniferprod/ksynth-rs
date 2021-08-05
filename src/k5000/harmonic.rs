use bit::BitIndex;
use crate::SystemExclusiveData;
use crate::k5000::morf::Loop;

pub struct Levels {
    pub soft: [u8; 64],
    pub loud: [u8; 64],
}

impl Default for Levels {
    fn default() -> Self {
        Levels {
            soft: [0; 64],
            loud: [0; 64],
        }
    }
}

impl SystemExclusiveData for Levels {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset = 0;

        let mut soft: [u8; 64] = [0; 64];
        for i in 0..64 {
            soft[i] = data[offset];
            offset += 1;
        }

        let mut loud: [u8; 64] = [0; 64];
        for i in 0..64 {
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct EnvelopeSegment {
    pub rate: u8,
    pub level: u8,
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: Vec<u8>) -> Self {
        EnvelopeSegment {
            rate: data[0],
            level: data[1],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate, self.level]
    }
}

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
        Envelope {
            segment0: EnvelopeSegment {
                rate: 0,
                level: 0,
            },
            segment1: EnvelopeSegment {
                rate: 0,
                level: 0,
            },
            segment2: EnvelopeSegment {
                rate: 0,
                level: 0,
            },
            segment3: EnvelopeSegment {
                rate: 0,
                level: 0,
            },
            loop_type: Loop::Off,
        }
    }

}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        let segment0_rate = data[0];
        let segment0_level = data[1];
        let segment1_rate = data[2];
        let segment1_level = data[3];
        let segment1_level_bit6 = data[3].bit(6);
        let segment2_rate = data[4];
        let mut segment2_level = data[5];
        let segment2_level_bit6 = data[5].bit(6);
        segment2_level.set_bit(6, false);
        let segment3_rate = data[6];
        let segment3_level = data[7];

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

        let mut segment1_level = self.segment1.level;
        let mut segment2_level = self.segment2.level;

        match self.loop_type {
            Loop::Loop1 => {
                segment1_level.set_bit(6, true);
                segment2_level.set_bit(6, true);
            },
            Loop::Loop2 => {
                segment1_level.set_bit(6, false);
                segment2_level.set_bit(6, true);
            },
            Loop::Off => {
                segment1_level.set_bit(6, false);
                segment2_level.set_bit(6, false);
            }
        }

        let mut segment1_data = self.segment1.to_bytes();
        segment1_data[1] = segment1_level;
        result.extend(segment1_data);

        let mut segment2_data = self.segment2.to_bytes();
        segment2_data[1] = segment2_level;
        result.extend(segment2_data);

        result.extend(self.segment3.to_bytes());

        result
    }
}
