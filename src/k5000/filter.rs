//! Data model for the filter (DCF).
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::{
    EnvelopeTime,
    EnvelopeLevel,
    ControlTime,
    EnvelopeDepth,
    Cutoff,
    Resonance,
    Level
};
use crate::k5000::control::VelocityCurve;

/// Filter mode.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
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
        Envelope {
            attack_time: EnvelopeTime::new(0),
            decay1_time: EnvelopeTime::new(0),
            decay1_level: EnvelopeLevel::new(0),
            decay2_time: EnvelopeTime::new(0),
            decay2_level: EnvelopeLevel::new(0),
            release_time: EnvelopeTime::new(0),
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
        Ok(Envelope {
            attack_time: EnvelopeTime::from(data[0]),
            decay1_time: EnvelopeTime::from(data[1]),
            decay1_level: EnvelopeLevel::from(data[2]),
            decay2_time: EnvelopeTime::from(data[3]),
            decay2_level: EnvelopeLevel::from(data[4]),
            release_time: EnvelopeTime::from(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.into(),
            self.decay1_time.into(),
            self.decay1_level.into(),
            self.decay2_time.into(),
            self.decay2_level.into(),
            self.release_time.into(),
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
        KeyScalingControl {
            attack_time: ControlTime::new(0),
            decay1_time: ControlTime::new(0),
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
        Ok(KeyScalingControl {
            attack_time: ControlTime::from(data[0]),
            decay1_time: ControlTime::from(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.into(),
            self.decay1_time.into(),
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
        VelocityControl {
            depth: EnvelopeDepth::new(0),
            attack_time: ControlTime::new(0),
            decay1_time: ControlTime::new(0),
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
        Ok(VelocityControl {
            depth: EnvelopeDepth::from(data[0]),
            attack_time: ControlTime::from(data[1]),
            decay1_time: ControlTime::from(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.depth.into(),
            self.attack_time.into(),
            self.decay1_time.into(),
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
        Ok(Modulation {
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
        Filter {
            is_active: true,
            cutoff: Cutoff::new(0),
            resonance: Resonance::new(0),
            mode: FilterMode::LowPass,
            velocity_curve: VelocityCurve::Curve1,
            level: Level::new(0),
            ks_to_cutoff: EnvelopeDepth::new(0),
            vel_to_cutoff: EnvelopeDepth::new(0),
            envelope_depth: EnvelopeDepth::new(0),
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
        Ok(Filter {
            is_active: data[0] != 1,  // value of 1 means filter is bypassed
            mode: FilterMode::try_from(data[1]).unwrap(),
            velocity_curve: VelocityCurve::try_from(data[2]).unwrap(),  // from 0 ~ 11 to enum
            resonance: Resonance::from(data[3]),
            level: Level::from(data[4]),
            cutoff: Cutoff::from(data[5]),
            ks_to_cutoff: EnvelopeDepth::from(data[6]),
            vel_to_cutoff: EnvelopeDepth::from(data[7]),
            envelope_depth: EnvelopeDepth::from(data[8]),
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
            self.resonance.into(),
            self.level.into(),
            self.cutoff.into(),
            self.ks_to_cutoff.into(),
            self.vel_to_cutoff.into(),
            self.envelope_depth.into(),
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
