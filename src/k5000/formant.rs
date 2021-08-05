use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::k5000::morf::Loop;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    Envelope,
    Lfo,
}

impl Default for Mode {
    fn default() -> Self { Mode::Envelope }
}

#[derive(Default)]
pub struct EnvelopeSegment {
    pub rate: u8,  // 0~127
    pub level: i8, // -63(1)~+63(127)
}

impl SystemExclusiveData for EnvelopeSegment {
    fn from_bytes(data: Vec<u8>) -> Self {
        EnvelopeSegment {
            rate: data[0],
            level: (data[1] - 64) as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.rate, (self.level + 64) as u8]
    }
}

pub struct Envelope {
    pub attack: EnvelopeSegment,
    pub decay1: EnvelopeSegment,
    pub decay2: EnvelopeSegment,
    pub release: EnvelopeSegment,
    pub decay_loop: Loop,
    pub velocity_depth: i8,
    pub ks_depth: i8,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            attack: Default::default(),
            decay1: Default::default(),
            decay2: Default::default(),
            release: Default::default(),
            decay_loop: Default::default(),
            velocity_depth: 0,
            ks_depth: 0,
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
            velocity_depth: (data[9] - 64) as i8,
            ks_depth: (data[10] - 64) as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.attack.to_bytes());
        result.extend(self.decay1.to_bytes());
        result.extend(self.decay2.to_bytes());
        result.extend(self.release.to_bytes());
        result.extend(vec![self.decay_loop as u8, (self.velocity_depth + 64) as u8, (self.ks_depth + 64) as u8]);

        result
    }
}

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

pub struct Lfo {
    pub speed: u8,
    pub shape: LfoShape,
    pub depth: u8,
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo {
            speed: 0,
            shape: Default::default(),
            depth: 0,
        }
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: Vec<u8>) -> Self {
        Lfo {
            speed: data[0],
            shape: LfoShape::try_from(data[1]).unwrap(),
            depth: data[2],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.speed, self.shape as u8, self.depth]
    }
}

pub struct FormantFilter {
    pub bias: i8,
    pub mode: Mode,
    pub envelope_depth: i8,
    pub envelope: Envelope,
    pub lfo: Lfo,
}

impl Default for FormantFilter {
    fn default() -> Self {
        FormantFilter {
            bias: 0,
            mode: Default::default(),
            envelope_depth: 0,
            envelope: Default::default(),
            lfo: Default::default(),
        }
    }
}

impl SystemExclusiveData for FormantFilter {
    fn from_bytes(data: Vec<u8>) -> Self {
        FormantFilter {
            bias: (data[0] - 64) as i8,
            mode: Mode::try_from(data[1]).unwrap(),
            envelope_depth: (data[2] - 64) as i8,
            envelope: Envelope::from_bytes(data[3..14].to_vec()),
            lfo: Lfo::from_bytes(data[14..].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(vec![(self.bias + 64) as u8, self.mode as u8, (self.envelope_depth + 64) as u8]);
        result.extend(self.envelope.to_bytes());
        result.extend(self.lfo.to_bytes());

        result
    }
}
