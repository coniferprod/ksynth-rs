//! Data model for the filter (DCF).
//!

use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

use crate::SystemExclusiveData;
use crate::k5000::{UnsignedLevel, SignedLevel, UnsignedDepth, SmallDepth};
use crate::k5000::control::VelocityCurve;

// Semantic types
pub type EnvelopeTime = UnsignedLevel;
pub type EnvelopeLevel = SignedLevel;
pub type KeyScalingLevel = SignedLevel;
pub type ControlTime = SignedLevel;
pub type VelocityControlLevel = UnsignedDepth;
pub type EnvelopeDepth = SignedLevel;
pub type Cutoff = UnsignedLevel;
pub type Resonance = SmallDepth;
pub type Level = SmallDepth;

/// Filter mode.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum FilterMode {
    LowPass = 0,
    HighPass = 1,
}

/// Filter envelope.
#[derive(Debug)]
pub struct Envelope {
    pub attack_time: EnvelopeTime,
    pub decay1_time: EnvelopeTime,
    pub decay1_level: EnvelopeLevel,
    pub decay2_time: EnvelopeTime,
    pub decay2_level: EnvelopeLevel,
    pub release_time: EnvelopeTime,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack_time: EnvelopeTime::from(0),
            decay1_time: EnvelopeTime::from(0),
            decay1_level: EnvelopeLevel::from(0),
            decay2_time: EnvelopeTime::from(0),
            decay2_level: EnvelopeLevel::from(0),
            release_time: EnvelopeTime::from(0),
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            attack_time: EnvelopeTime::from(data[0]),
            decay1_time: EnvelopeTime::from(data[1]),
            decay1_level: EnvelopeLevel::from(data[2]),
            decay2_time: EnvelopeTime::from(data[3]),
            decay2_level: EnvelopeLevel::from(data[4]),
            release_time: EnvelopeTime::from(data[5]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
            self.decay1_level.as_byte(),
            self.decay2_time.as_byte(),
            self.decay2_level.as_byte(),
            self.release_time.as_byte(),
        ];
        result.extend(bs);

        result
    }
}

/// Filter key scaling control.
pub struct KeyScalingControl {
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        KeyScalingControl {
            attack_time: ControlTime::from(0),
            decay1_time: ControlTime::from(0),
        }
    }
}

impl SystemExclusiveData for KeyScalingControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        KeyScalingControl {
            attack_time: ControlTime::from(data[1]),
            decay1_time: ControlTime::from(data[2]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
        ]
    }
}

/// Filter velocity control.
pub struct VelocityControl {
    pub depth: EnvelopeDepth,
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
}

impl Default for VelocityControl {
    fn default() -> Self {
        VelocityControl {
            depth: EnvelopeDepth::from(0),
            attack_time: ControlTime::from(0),
            decay1_time: ControlTime::from(0),
        }
    }
}

impl SystemExclusiveData for VelocityControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        VelocityControl {
            depth: EnvelopeDepth::from(data[0]),
            attack_time: ControlTime::from(data[1]),
            decay1_time: ControlTime::from(data[2]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.depth.as_byte(),
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
        ]
    }
}

/// Modulation settings for the filter.
pub struct Modulation {
    pub ks_to_env: KeyScalingControl,
    pub vel_to_env: VelocityControl,
}

impl Default for Modulation {
    fn default() -> Self {
        Modulation {
            ks_to_env: Default::default(),
            vel_to_env: Default::default(),
        }
    }
}

impl SystemExclusiveData for Modulation {
    fn from_bytes(data: Vec<u8>) -> Self {
        Modulation {
            ks_to_env: KeyScalingControl::from_bytes(data[..2].to_vec()),
            vel_to_env: VelocityControl::from_bytes(data[2..5].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_to_env.to_bytes());

        result
    }
}


/// Filter settings.
pub struct Filter {
    pub is_active: bool,
    pub cutoff: Cutoff,
    pub resonance: Resonance,
    pub mode: FilterMode,
    pub velocity_curve: VelocityCurve,
    pub level: Level,
    pub ks_to_cutoff: EnvelopeDepth,
    pub vel_to_cutoff: EnvelopeDepth,
    pub envelope_depth: EnvelopeDepth,
    pub envelope: Envelope,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            is_active: true,
            cutoff: Cutoff::from(0),
            resonance: Resonance::from(0),
            mode: FilterMode::LowPass,
            velocity_curve: VelocityCurve::Curve1,
            level: Level::from(0),
            ks_to_cutoff: EnvelopeDepth::from(0),
            vel_to_cutoff: EnvelopeDepth::from(0),
            envelope_depth: EnvelopeDepth::from(0),
            envelope: Envelope::new(),
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::new()
    }
}

impl SystemExclusiveData for Filter {
    fn from_bytes(data: Vec<u8>) -> Self {
        Filter {
            is_active: if data[0] == 1 { false } else { true },  // value of 1 means filter is bypassed
            mode: FilterMode::try_from(data[1]).unwrap(),
            velocity_curve: VelocityCurve::try_from(data[2]).unwrap(),  // from 0 ~ 11 to enum
            resonance: Resonance::from(data[3]),
            level: Level::from(data[4]),
            cutoff: Cutoff::from(data[5]),
            ks_to_cutoff: EnvelopeDepth::from(data[6]),
            vel_to_cutoff: EnvelopeDepth::from(data[7]),
            envelope_depth: EnvelopeDepth::from(data[8]),
            envelope: Envelope::from_bytes(data[9..].to_vec())
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            if self.is_active { 0 } else { 1 },  // is this the right way around?
            self.mode as u8,
            self.velocity_curve as u8,  // raw enum values map to 0~11
            self.resonance.as_byte(),
            self.level.as_byte(),
            self.cutoff.as_byte(),
            self.ks_to_cutoff.as_byte(),
            self.vel_to_cutoff.as_byte(),
            self.envelope_depth.as_byte(),
        ];
        result.extend(bs);
        result.extend(self.envelope.to_bytes());

        result
    }
}
