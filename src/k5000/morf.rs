//! Data model for MORF.
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
    ByteValue, DESCRIPTORS
};
use crate::k5000::control::VelocityCurve;

/// Harmonic group.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmonicCommon {
    pub morf_enabled: bool,
    pub total_gain: u8,
    pub group: HarmonicGroup,
    pub ks_to_gain: i32, // KeyScalingToGain,
    pub velocity_curve: VelocityCurve,
    pub velocity_depth: i32, // VelocityDepth,
}

impl Default for HarmonicCommon {
    fn default() -> Self {
        HarmonicCommon {
            morf_enabled: false,
            total_gain: 0,
            group: Default::default(),
            ks_to_gain: Default::default(),
            velocity_curve: VelocityCurve::Curve1,
            velocity_depth: Default::default(),
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
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScalingToGain).unwrap();
        let vd_desc = DESCRIPTORS.get(&ByteValue::VelocityDepth).unwrap();

        Ok(Self {
            morf_enabled: data[0] == 1,
            total_gain: data[1],
            group: HarmonicGroup::try_from(data[2]).unwrap(),
            ks_to_gain: (ks_desc.incoming)(data[3]),
            velocity_curve: VelocityCurve::try_from(data[4]).unwrap(), // 0~11 maps to enum
            velocity_depth: (vd_desc.incoming)(data[5]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScalingToGain).unwrap();
        let vd_desc = DESCRIPTORS.get(&ByteValue::VelocityDepth).unwrap();

        vec![
            if self.morf_enabled { 1 } else { 0 },
            self.total_gain,
            self.group as u8,
            (ks_desc.outgoing)(self.ks_to_gain),
            self.velocity_curve as u8,
            (vd_desc.outgoing)(self.velocity_depth),
        ]
    }

    fn data_size() -> usize { 6 }
}

/// MORF harmonic copy parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        Ok(Self {
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default, Serialize, Deserialize)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorfHarmonicEnvelope {
    pub times: [i32; 4], // EnvelopeTime
    /*
    pub time1: EnvelopeTime,
    pub time2: EnvelopeTime,
    pub time3: EnvelopeTime,
    pub time4: EnvelopeTime,
     */
    pub loop_type: Loop,
}

impl Default for MorfHarmonicEnvelope {
    fn default() -> Self {
        Self {
            times: [Default::default(); 4],
            /*
            time1: Default::default(),
            time2: Default::default(),
            time3: Default::default(),
            time4: Default::default(),
             */
            loop_type: Default::default(),
        }
    }
}

impl fmt::Display for MorfHarmonicEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Time1={} Time2={} Time3={} Time4={} Loop={}",
            self.times[0], self.times[1], self.times[2], self.times[3],
            self.loop_type)
    }
}

impl SystemExclusiveData for MorfHarmonicEnvelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let time_desc = DESCRIPTORS.get(&ByteValue::EnvelopeTime).unwrap();
        Ok(Self {
            times: [
                (time_desc.incoming)(data[0]),
                (time_desc.incoming)(data[1]),
                (time_desc.incoming)(data[2]),
                (time_desc.incoming)(data[3]),
            ],
            loop_type: Loop::try_from(data[4]).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let time_desc = DESCRIPTORS.get(&ByteValue::EnvelopeTime).unwrap();
        vec![
            (time_desc.outgoing)(self.times[0]),
            (time_desc.outgoing)(self.times[1]),
            (time_desc.outgoing)(self.times[2]),
            (time_desc.outgoing)(self.times[3]),
            self.loop_type as u8,
        ]
    }

    fn data_size() -> usize { 5 }
}

/// MORF harmonic settings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        Ok(Self {
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
        4 * MorfHarmonicCopyParameters::data_size() 
        + MorfHarmonicEnvelope::data_size()
    }
}
