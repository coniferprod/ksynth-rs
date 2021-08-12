//! Data model for MORF.
//!

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::k5000::{RangedValue, RangeKind};

/// Harmonic group.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum HarmonicGroup {
    Low,
    High
}

impl Default for HarmonicGroup {
    fn default() -> Self { HarmonicGroup::Low }
}

pub struct HarmonicCommon {
    pub morf_enabled: bool,
    pub total_gain: u8,
    pub group: HarmonicGroup,
    pub ks_to_gain: RangedValue,
    pub velocity_curve: RangedValue,
    pub velocity_depth: u8,
}

impl Default for HarmonicCommon {
    fn default() -> Self {
        HarmonicCommon {
            morf_enabled: false,
            total_gain: 0,
            group: Default::default(),
            ks_to_gain: RangedValue::from_int(RangeKind::SignedLevel, 0),
            velocity_curve: RangedValue::from_int(RangeKind::VelocityCurve, 1),
            velocity_depth: 0,
        }
    }
}

impl SystemExclusiveData for HarmonicCommon {
    fn from_bytes(data: Vec<u8>) -> Self {
        HarmonicCommon {
            morf_enabled: data[0] == 1,
            total_gain: data[1],
            group: HarmonicGroup::try_from(data[2]).unwrap(),
            ks_to_gain: RangedValue::from_byte(RangeKind::SignedLevel, data[3]),
            velocity_curve: RangedValue::from_byte(RangeKind::VelocityCurve, data[4] + 1), // 0~11 to 1~12
            velocity_depth: data[5],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            if self.morf_enabled { 1 } else { 0 },
            self.total_gain,
            self.group as u8,
            self.ks_to_gain.as_byte(),
            self.velocity_curve.as_byte(),
            self.velocity_depth,
        ]
    }
}

/// MORF harmonic copy parameters.
pub struct MorfHarmonicCopyParameters {
    pub patch_number: u8,
    pub source_number: u8,
}

impl Default for MorfHarmonicCopyParameters {
    fn default() -> Self {
        MorfHarmonicCopyParameters {
            patch_number: 0,
            source_number: 0,
        }
    }
}

impl SystemExclusiveData for MorfHarmonicCopyParameters {
    fn from_bytes(data: Vec<u8>) -> Self {
        MorfHarmonicCopyParameters {
            patch_number: data[0],
            source_number: data[1],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.patch_number, self.source_number]
    }
}

/// MORF harmonic envelope loop type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Loop {
    Off,
    Loop1,
    Loop2,
}

impl Default for Loop {
    fn default() -> Self { Loop::Off }
}

/// MORF harmonic envelope.
pub struct MorfHarmonicEnvelope {
    pub time1: RangedValue,
    pub time2: RangedValue,
    pub time3: RangedValue,
    pub time4: RangedValue,
    pub loop_type: Loop,
}

impl Default for MorfHarmonicEnvelope {
    fn default() -> Self {
        MorfHarmonicEnvelope {
            time1: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            time2: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            time3: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            time4: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            loop_type: Default::default(),
        }
    }
}

impl SystemExclusiveData for MorfHarmonicEnvelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        MorfHarmonicEnvelope {
            time1: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            time2: RangedValue::from_byte(RangeKind::PositiveLevel, data[1]),
            time3: RangedValue::from_byte(RangeKind::PositiveLevel, data[2]),
            time4: RangedValue::from_byte(RangeKind::PositiveLevel, data[3]),
            loop_type: Loop::try_from(data[4]).unwrap(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.time1.as_byte(),
            self.time2.as_byte(),
            self.time3.as_byte(),
            self.time4.as_byte(),
            self.loop_type as u8,
        ]
    }
}

/// MORF harmonic settings.
pub struct MorfHarmonic {
    pub copy1: MorfHarmonicCopyParameters,
    pub copy2: MorfHarmonicCopyParameters,
    pub copy3: MorfHarmonicCopyParameters,
    pub copy4: MorfHarmonicCopyParameters,
    pub envelope: MorfHarmonicEnvelope,
}

impl Default for MorfHarmonic {
    fn default() -> Self {
        MorfHarmonic {
            copy1: Default::default(),
            copy2: Default::default(),
            copy3: Default::default(),
            copy4: Default::default(),
            envelope: Default::default(),
        }
    }
}

impl SystemExclusiveData for MorfHarmonic {
    fn from_bytes(data: Vec<u8>) -> Self {
        MorfHarmonic {
            copy1: MorfHarmonicCopyParameters::from_bytes(data[..2].to_vec()),
            copy2: MorfHarmonicCopyParameters::from_bytes(data[2..4].to_vec()),
            copy3: MorfHarmonicCopyParameters::from_bytes(data[4..6].to_vec()),
            copy4: MorfHarmonicCopyParameters::from_bytes(data[6..8].to_vec()),
            envelope: MorfHarmonicEnvelope::from_bytes(data[8..].to_vec()),
        }
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
