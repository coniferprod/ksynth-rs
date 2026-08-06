//! Data model for the filter (DCF).
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use rand::Rng;

use crate::{
    SystemExclusiveData,
    ParseError,
    Ranged,
    ranged_impl,
    Adjustment,
    parse_or_default,
};
use crate::k5000::{
    EnvelopeTime,
    EnvelopeLevel,
    ControlTime,
    EnvelopeDepth,
};
use crate::k5000::control::VelocityCurve;

/// Cutoff (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cutoff(i32);
ranged_impl!(Cutoff, 0, 127, 0);

impl Adjustment for Cutoff { }

/// Resonance (0...31, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Resonance(i32);
ranged_impl!(Resonance, 0, 31, 0);

impl Adjustment for Resonance { }

/// Level (0...31, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Level(i32);
ranged_impl!(Level, 0, 31, 0);

impl Adjustment for Level { }

/// Filter mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum FilterMode {
    LowPass = 0,
    HighPass = 1,
}

impl fmt::Display for FilterMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            FilterMode::LowPass => String::from("Low pass"),
            FilterMode::HighPass => String::from("High pass"),
        })
    }
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
        Self {
            attack_time: Default::default(),
            decay1_time: Default::default(),
            decay1_level: Default::default(),
            decay2_time: Default::default(),
            decay2_level: Default::default(),
            release_time: Default::default(),
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A={} D1={}/{} D2={}/{} R={}",
            self.attack_time, self.decay1_time, self.decay1_level,
            self.decay2_time, self.decay2_level, self.release_time
        )
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            attack_time: parse_or_default::<EnvelopeTime>(data[0]),
            decay1_time: parse_or_default::<EnvelopeTime>(data[1]),
            decay1_level: parse_or_default::<EnvelopeLevel>(data[2]),
            decay2_time: parse_or_default::<EnvelopeTime>(data[3]),
            decay2_level: parse_or_default::<EnvelopeLevel>(data[4]),
            release_time: parse_or_default::<EnvelopeTime>(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.outgoing(),
            self.decay1_time.outgoing(),
            self.decay1_level.outgoing(),
            self.decay2_time.outgoing(),
            self.decay2_level.outgoing(),
            self.release_time.outgoing(),
        ]
    }

    fn data_size() -> usize { 6 }
}

/// Filter key scaling control.
#[derive(Debug)]
pub struct KeyScalingControl {
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        Self {
            attack_time: Default::default(),
            decay1_time: Default::default(),
        }
    }
}

impl fmt::Display for KeyScalingControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atk={} Dcy1={}", self.attack_time, self.decay1_time)
    }
}

impl SystemExclusiveData for KeyScalingControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            attack_time: parse_or_default::<ControlTime>(data[0]),
            decay1_time: parse_or_default::<ControlTime>(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.outgoing(),
            self.decay1_time.outgoing(),
        ]
    }

    fn data_size() -> usize { 2 }
}

/// Filter velocity control.
#[derive(Debug)]
pub struct VelocityControl {
    pub depth: EnvelopeDepth,
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
}

impl Default for VelocityControl {
    fn default() -> Self {
        Self {
            depth: Default::default(),
            attack_time: Default::default(),
            decay1_time: Default::default(),
        }
    }
}

impl fmt::Display for VelocityControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Depth={} Atk={} Dcy1={}",
            self.depth, self.attack_time, self.decay1_time)
    }
}

impl SystemExclusiveData for VelocityControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            depth: parse_or_default::<EnvelopeDepth>(data[0]),
            attack_time: parse_or_default::<ControlTime>(data[1]),
            decay1_time: parse_or_default::<ControlTime>(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.depth.outgoing(),
            self.attack_time.outgoing(),
            self.decay1_time.outgoing(),
        ]
    }

    fn data_size() -> usize { 3 }
}

/// Modulation settings for the filter.
#[derive(Default, Debug)]
pub struct Modulation {
    pub ks_to_env: KeyScalingControl,
    pub vel_to_env: VelocityControl,
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KS->Env={} Vel->Env={}", self.ks_to_env, self.vel_to_env)
    }
}

impl SystemExclusiveData for Modulation {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            ks_to_env: KeyScalingControl::from_bytes(&data[..2])?,
            vel_to_env: VelocityControl::from_bytes(&data[2..5])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_to_env.to_bytes());

        result
    }

    fn data_size() -> usize {
        KeyScalingControl::data_size() + VelocityControl::data_size()
    }
}


/// Filter settings.
#[derive(Debug)]
pub struct Filter {
    pub is_active: bool,
    pub mode: FilterMode,
    pub velocity_curve: VelocityCurve,
    pub resonance: Resonance,
    pub level: Level,
    pub cutoff: Cutoff,
    pub ks_to_cutoff: EnvelopeDepth,
    pub vel_to_cutoff: EnvelopeDepth,
    pub envelope_depth: EnvelopeDepth,
    pub envelope: Envelope,
    pub modulation: Modulation,
}

impl Filter {
    pub fn new() -> Filter {
        Self {
            is_active: true,
            cutoff: Default::default(),
            resonance: Default::default(),
            mode: FilterMode::LowPass,
            velocity_curve: VelocityCurve::Curve1,
            level: Default::default(),
            ks_to_cutoff: Default::default(),
            vel_to_cutoff: Default::default(),
            envelope_depth: Default::default(),
            envelope: Envelope::new(),
            modulation: Modulation::default()
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::new()
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Active={} Cutoff={} Resonance={} Mode={}\nVel Curve={} Level=0{}\nKS to Cutoff={} Vel. to Cutoff={} Env Depth={}\nEnvelope: {}\nModulation: {}",
            self.is_active, self.cutoff, self.resonance,
            self.mode, self.velocity_curve, self.level,
            self.ks_to_cutoff, self.vel_to_cutoff, self.envelope_depth,
            self.envelope, self.modulation
        )
    }
}

impl SystemExclusiveData for Filter {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            is_active: data[0] != 1,  // value of 1 means filter is bypassed
            mode: FilterMode::try_from(data[1]).unwrap(),
            velocity_curve: VelocityCurve::try_from(data[2]).unwrap(),  // from 0 ~ 11 to enum
            resonance: parse_or_default::<Resonance>(data[3]),
            level: parse_or_default::<Level>(data[4]),
            cutoff: parse_or_default::<Cutoff>(data[5]),
            ks_to_cutoff: parse_or_default::<EnvelopeDepth>(data[6]),
            vel_to_cutoff: parse_or_default::<EnvelopeDepth>(data[7]),
            envelope_depth: parse_or_default::<EnvelopeDepth>(data[8]),
            envelope: Envelope::from_bytes(&data[9..15])?,
            modulation: Modulation::from_bytes(&data[15..20])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            if self.is_active { 0 } else { 1 },  // is this the right way around?
            self.mode as u8,
            self.velocity_curve as u8,  // raw enum values map to 0~11
            self.resonance.outgoing(),
            self.level.outgoing(),
            self.cutoff.outgoing(),
            self.ks_to_cutoff.outgoing(),
            self.vel_to_cutoff.outgoing(),
            self.envelope_depth.outgoing(),
        ];
        result.extend(bs);
        result.extend(self.envelope.to_bytes());
        result.extend(self.modulation.to_bytes());

        result
    }

    fn data_size() -> usize {
        9
        + Envelope::data_size()
        + Modulation::data_size()
    }
}
