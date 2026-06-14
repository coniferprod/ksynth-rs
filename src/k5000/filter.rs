//! Data model for the filter (DCF).
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use serde::{Serialize, Deserialize};

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::{
    ByteValue,
    DESCRIPTORS,
};
use crate::k5000::control::VelocityCurve;

/// Filter mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    pub attack_time: i32, // EnvelopeTime,
    pub decay1_time: i32, // EnvelopeTime,
    pub decay1_level: i32, // EnvelopeLevel,
    pub decay2_time: i32, // EnvelopeTime,
    pub decay2_level: i32, // EnvelopeLevel,
    pub release_time: i32, // EnvelopeTime,
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
        let time_desc = DESCRIPTORS.get(&ByteValue::EnvelopeTime).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeLevel).unwrap();

        Ok(Self {
            attack_time: (time_desc.incoming)(data[0]),
            decay1_time: (time_desc.incoming)(data[1]),
            decay1_level: (level_desc.incoming)(data[2]),
            decay2_time: (time_desc.incoming)(data[3]),
            decay2_level: (level_desc.incoming)(data[4]),
            release_time: (time_desc.incoming)(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let time_desc = DESCRIPTORS.get(&ByteValue::EnvelopeTime).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::PitchEnvelopeLevel).unwrap();

        vec![
            (time_desc.outgoing)(self.attack_time),
            (time_desc.outgoing)(self.decay1_time),
            (level_desc.outgoing)(self.decay1_level),
            (time_desc.outgoing)(self.decay2_time),
            (level_desc.outgoing)(self.decay2_level),
            (time_desc.outgoing)(self.release_time),
        ]
    }

    fn data_size() -> usize { 6 }
}

/// Filter key scaling control.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyScalingControl {
    pub attack_time: i32, // ControlTime,
    pub decay1_time: i32, // ControlTime,
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
        let desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        Ok(Self {
            attack_time: (desc.incoming)(data[0]),
            decay1_time: (desc.incoming)(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        vec![
            (desc.outgoing)(self.attack_time),
            (desc.outgoing)(self.decay1_time),
        ]
    }

    fn data_size() -> usize { 2 }
}

/// Filter velocity control.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VelocityControl {
    pub depth: i32, // EnvelopeDepth,
    pub attack_time: i32, // ControlTime,
    pub decay1_time: i32, // ControlTime,
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
        let e_desc = DESCRIPTORS.get(&ByteValue::EnvelopeDepth).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        Ok(Self {
            depth: (e_desc.incoming)(data[0]),
            attack_time: (ct_desc.incoming)(data[1]),
            decay1_time: (ct_desc.incoming)(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let e_desc = DESCRIPTORS.get(&ByteValue::EnvelopeDepth).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        vec![
            (e_desc.outgoing)(self.depth),
            (ct_desc.outgoing)(self.attack_time),
            (ct_desc.outgoing)(self.decay1_time),
        ]
    }

    fn data_size() -> usize { 3 }
}

/// Modulation settings for the filter.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    pub is_active: bool,
    pub mode: FilterMode,
    pub velocity_curve: VelocityCurve,
    pub resonance: i32, // Resonance,
    pub level: i32, // Level,
    pub cutoff: i32, // Cutoff,
    pub ks_to_cutoff: i32, // EnvelopeDepth,
    pub vel_to_cutoff: i32, // EnvelopeDepth,
    pub envelope_depth: i32, // EnvelopeDepth,
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
            modulation: Default::default()
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
        let res_desc = DESCRIPTORS.get(&ByteValue::Resonance).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::Level).unwrap();
        let cutoff_desc = DESCRIPTORS.get(&ByteValue::Cutoff).unwrap();
        let ed_desc = DESCRIPTORS.get(&ByteValue::EnvelopeDepth).unwrap();

        Ok(Self {
            is_active: data[0] != 1,  // value of 1 means filter is bypassed
            mode: FilterMode::try_from(data[1]).unwrap(),
            velocity_curve: VelocityCurve::try_from(data[2]).unwrap(),  // from 0 ~ 11 to enum
            resonance: (res_desc.incoming)(data[3]),
            level: (level_desc.incoming)(data[4]),
            cutoff: (cutoff_desc.incoming)(data[5]),
            ks_to_cutoff: (ed_desc.incoming)(data[6]),
            vel_to_cutoff: (ed_desc.incoming)(data[7]),
            envelope_depth: (ed_desc.incoming)(data[8]),
            envelope: Envelope::from_bytes(&data[9..15])?,
            modulation: Modulation::from_bytes(&data[15..20])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let res_desc = DESCRIPTORS.get(&ByteValue::Resonance).unwrap();
        let level_desc = DESCRIPTORS.get(&ByteValue::Level).unwrap();
        let cutoff_desc = DESCRIPTORS.get(&ByteValue::Cutoff).unwrap();
        let ed_desc = DESCRIPTORS.get(&ByteValue::EnvelopeDepth).unwrap();

        let bs = vec![
            if self.is_active { 0 } else { 1 },  // is this the right way around?
            self.mode as u8,
            self.velocity_curve as u8,  // raw enum values map to 0~11
            (res_desc.outgoing)(self.resonance),
            (level_desc.outgoing)(self.level),
            (cutoff_desc.outgoing)(self.cutoff),
            (ed_desc.outgoing)(self.ks_to_cutoff),
            (ed_desc.outgoing)(self.vel_to_cutoff),
            (ed_desc.outgoing)(self.envelope_depth),
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
