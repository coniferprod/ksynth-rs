//! Data model for MORF.
//!

use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

use crate::{
    SystemExclusiveData, 
    ParseError
};
use crate::k5000::{
    VelocityDepth, 
    EnvelopeTime, 
    KeyScalingToGain
};
use crate::k5000::control::VelocityCurve;

/// Harmonic group.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum HarmonicGroup {
    #[default]
    Low,

    High
}

/// Harmonic common settings.
pub struct HarmonicCommon {
    pub morf_enabled: bool,
    pub total_gain: u8,
    pub group: HarmonicGroup,
    pub ks_to_gain: KeyScalingToGain,
    pub velocity_curve: VelocityCurve,
    pub velocity_depth: VelocityDepth,
}

impl Default for HarmonicCommon {
    fn default() -> Self {
        HarmonicCommon {
            morf_enabled: false,
            total_gain: 0,
            group: Default::default(),
            ks_to_gain: KeyScalingToGain::new(0),
            velocity_curve: VelocityCurve::Curve1,
            velocity_depth: VelocityDepth::new(0),
        }
    }
}

impl SystemExclusiveData for HarmonicCommon {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(HarmonicCommon {
            morf_enabled: data[0] == 1,
            total_gain: data[1],
            group: HarmonicGroup::try_from(data[2]).unwrap(),
            ks_to_gain: KeyScalingToGain::from(data[3]),
            velocity_curve: VelocityCurve::try_from(data[4]).unwrap(), // 0~11 maps to enum
            velocity_depth: VelocityDepth::from(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            if self.morf_enabled { 1 } else { 0 },
            self.total_gain,
            self.group as u8,
            self.ks_to_gain.into(),
            self.velocity_curve as u8,
            self.velocity_depth.into(),
        ]
    }
}

/// MORF harmonic copy parameters.
#[derive(Default)]
pub struct MorfHarmonicCopyParameters {
    pub patch_number: u8,
    pub source_number: u8,
}

impl SystemExclusiveData for MorfHarmonicCopyParameters {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(MorfHarmonicCopyParameters {
            patch_number: data[0],
            source_number: data[1],
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.patch_number, self.source_number]
    }
}

/// MORF harmonic envelope loop type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum Loop {
    #[default]
    Off,

    Loop1,
    Loop2,
}

/// MORF harmonic envelope.
pub struct MorfHarmonicEnvelope {
    pub time1: EnvelopeTime,
    pub time2: EnvelopeTime,
    pub time3: EnvelopeTime,
    pub time4: EnvelopeTime,
    pub loop_type: Loop,
}

impl Default for MorfHarmonicEnvelope {
    fn default() -> Self {
        MorfHarmonicEnvelope {
            time1: EnvelopeTime::new(0),
            time2: EnvelopeTime::new(0),
            time3: EnvelopeTime::new(0),
            time4: EnvelopeTime::new(0),
            loop_type: Default::default(),
        }
    }
}

impl SystemExclusiveData for MorfHarmonicEnvelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(MorfHarmonicEnvelope {
            time1: EnvelopeTime::from(data[0]),
            time2: EnvelopeTime::from(data[1]),
            time3: EnvelopeTime::from(data[2]),
            time4: EnvelopeTime::from(data[3]),
            loop_type: Loop::try_from(data[4]).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.time1.into(),
            self.time2.into(),
            self.time3.into(),
            self.time4.into(),
            self.loop_type as u8,
        ]
    }
}

/// MORF harmonic settings.
#[derive(Default)]
pub struct MorfHarmonic {
    pub copy1: MorfHarmonicCopyParameters,
    pub copy2: MorfHarmonicCopyParameters,
    pub copy3: MorfHarmonicCopyParameters,
    pub copy4: MorfHarmonicCopyParameters,
    pub envelope: MorfHarmonicEnvelope,
}

impl SystemExclusiveData for MorfHarmonic {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(MorfHarmonic {
            copy1: MorfHarmonicCopyParameters::from_bytes(&data[..2])?,
            copy2: MorfHarmonicCopyParameters::from_bytes(&data[2..4])?,
            copy3: MorfHarmonicCopyParameters::from_bytes(&data[4..6])?,
            copy4: MorfHarmonicCopyParameters::from_bytes(&data[6..8])?,
            envelope: MorfHarmonicEnvelope::from_bytes(&data[8..])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.copy1.to_bytes());
        result.extend(self.copy2.to_bytes());
        result.extend(self.copy3.to_bytes());
        result.extend(self.copy4.to_bytes());
        result.extend(self.envelope.to_bytes());

        result
    }
}
