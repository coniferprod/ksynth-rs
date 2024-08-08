//! Data model for MORF.
//!

use std::convert::TryFrom;
use std::fmt;

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

impl fmt::Display for HarmonicGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if *self == HarmonicGroup::Low { "LO" } else { "HI" })
    }
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

impl fmt::Display for HarmonicCommon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MORF enabled={} Total gain={} Group={} KStoGain={} VelCurve={} VelDepth={}",
            self.morf_enabled, self.total_gain, self.group,
            self.ks_to_gain, self.velocity_curve, self.velocity_depth)
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

    fn data_size() -> usize { 6 }
}

/// MORF harmonic copy parameters.
#[derive(Default)]
pub struct MorfHarmonicCopyParameters {
    pub patch_number: u8,
    pub source_number: u8,
}

impl fmt::Display for MorfHarmonicCopyParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PatchNo={} SourceNo={}",
            self.patch_number, self.source_number)
    }
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

    fn data_size() -> usize { 2 }
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

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Loop::Off => "Off",
            Loop::Loop1 => "Loop1",
            Loop::Loop2 => "Loop2",
        })
    }
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

impl fmt::Display for MorfHarmonicEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Time1={} Time2={} Time3={} Time4={} Loop={}",
            self.time1, self.time2, self.time3, self.time4,
            self.loop_type)
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

    fn data_size() -> usize { 5 }
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

impl fmt::Display for MorfHarmonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Copy1={} Copy2={} Copy3={} Copy4={} Envelope={}",
            self.copy1, self.copy2, self.copy3, self.copy4,
            self.envelope)
    }
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

    fn data_size() -> usize {
        4 * MorfHarmonicCopyParameters::data_size() + MorfHarmonicEnvelope::data_size()
    }
}
