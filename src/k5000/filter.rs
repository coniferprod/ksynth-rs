//! Data model for the filter (DCF).
//!

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::k5000::{RangedValue, RangeKind};

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
    pub attack_time: RangedValue,
    pub decay1_time: RangedValue,
    pub decay1_level: RangedValue,
    pub decay2_time: RangedValue,
    pub decay2_level: RangedValue,
    pub release_time: RangedValue,
    pub ks_to_attack: RangedValue,
    pub ks_to_decay1: RangedValue,
    pub vel_to_envelope: RangedValue,
    pub vel_to_attack: RangedValue,
    pub vel_to_decay1: RangedValue,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay1_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay1_level: RangedValue::from_int(RangeKind::SignedLevel, 0),
            decay2_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay2_level: RangedValue::from_int(RangeKind::SignedLevel, 0),
            release_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            ks_to_attack: RangedValue::from_int(RangeKind::SignedLevel, 0),
            ks_to_decay1: RangedValue::from_int(RangeKind::SignedLevel, 0),
            vel_to_envelope: RangedValue::from_int(RangeKind::SignedLevel, 0),
            vel_to_attack: RangedValue::from_int(RangeKind::SignedLevel, 0),
            vel_to_decay1: RangedValue::from_int(RangeKind::SignedLevel, 0),
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
            attack_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            decay1_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[1]),
            decay1_level: RangedValue::from_byte(RangeKind::SignedLevel, data[2]),
            decay2_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[3]),
            decay2_level: RangedValue::from_byte(RangeKind::SignedLevel, data[4]),
            release_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[5]),
            ks_to_attack: RangedValue::from_byte(RangeKind::SignedLevel, data[6]),
            ks_to_decay1: RangedValue::from_byte(RangeKind::SignedLevel, data[7]),
            vel_to_envelope: RangedValue::from_byte(RangeKind::SignedLevel, data[8]),
            vel_to_attack: RangedValue::from_byte(RangeKind::SignedLevel, data[9]),
            vel_to_decay1: RangedValue::from_byte(RangeKind::SignedLevel, data[10]),
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
            self.ks_to_attack.as_byte(),
            self.ks_to_decay1.as_byte(),
            self.vel_to_envelope.as_byte(),
            self.vel_to_attack.as_byte(),
            self.vel_to_decay1.as_byte(),
        ];
        result.extend(bs);

        result
    }
}

/// Filter settings.
pub struct Filter {
    pub is_active: bool,
    pub cutoff: RangedValue,
    pub resonance: RangedValue,
    pub mode: FilterMode,
    pub velocity_curve: RangedValue,
    pub level: RangedValue,
    pub ks_to_cutoff: RangedValue,
    pub vel_to_cutoff: RangedValue,
    pub envelope_depth: RangedValue,
    pub envelope: Envelope,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            is_active: true,
            cutoff: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            resonance: RangedValue::from_int(RangeKind::FilterResonance, 0),
            mode: FilterMode::LowPass,
            velocity_curve: RangedValue::from_int(RangeKind::VelocityCurve, 1),
            level: RangedValue::from_int(RangeKind::FilterLevel, 0),
            ks_to_cutoff: RangedValue::from_int(RangeKind::SignedLevel, 0),
            vel_to_cutoff: RangedValue::from_int(RangeKind::SignedLevel, 0),
            envelope_depth: RangedValue::from_int(RangeKind::SignedLevel, 0),
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
            velocity_curve: RangedValue::from_byte(RangeKind::VelocityCurve, data[2] + 1),  // adjust from 0 ~ 11 to 1 ~ 12
            resonance: RangedValue::from_byte(RangeKind::FilterResonance, data[3]),
            level: RangedValue::from_byte(RangeKind::FilterLevel, data[4]),
            cutoff: RangedValue::from_byte(RangeKind::PositiveLevel, data[5]),
            ks_to_cutoff: RangedValue::from_byte(RangeKind::SignedLevel, data[6]),
            vel_to_cutoff: RangedValue::from_byte(RangeKind::SignedLevel, data[7]),
            envelope_depth: RangedValue::from_byte(RangeKind::SignedLevel, data[8]),
            envelope: Envelope::from_bytes(data[9..].to_vec())
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            if self.is_active { 0 } else { 1 },  // is this the right way around?
            self.mode as u8,
            self.velocity_curve.as_byte() - 1,  // adjust from 1~12 to 0~11
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
