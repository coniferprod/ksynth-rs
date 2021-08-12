//! Data model for the amplifier (DCA).
//!

use crate::SystemExclusiveData;
use crate::k5000::{RangedValue, RangeKind};

/// Amplifier envelope.
#[derive(Debug)]
pub struct Envelope {
    pub attack_time: RangedValue,
    pub decay1_time: RangedValue,
    pub decay1_level: RangedValue,
    pub decay2_time: RangedValue,
    pub decay2_level: RangedValue,
    pub release_time: RangedValue,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay1_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay1_level: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay2_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            decay2_level: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            release_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
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
            decay1_level: RangedValue::from_byte(RangeKind::PositiveLevel, data[2]),
            decay2_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[3]),
            decay2_level: RangedValue::from_byte(RangeKind::PositiveLevel, data[4]),
            release_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[5]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
            self.decay1_level.as_byte(),
            self.decay2_time.as_byte(),
            self.decay2_level.as_byte(),
            self.release_time.as_byte()
        ]
    }
}

/// Key scaling control.
pub struct KeyScalingControl {
    pub level: RangedValue,
    pub attack_time: RangedValue,
    pub decay1_time: RangedValue,
    pub release: RangedValue,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        KeyScalingControl {
            level: RangedValue::from_int(RangeKind::SignedLevel, 0),
            attack_time: RangedValue::from_int(RangeKind::SignedLevel, 0),
            decay1_time: RangedValue::from_int(RangeKind::SignedLevel, 0),
            release: RangedValue::from_int(RangeKind::SignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for KeyScalingControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        KeyScalingControl {
            level: RangedValue::from_byte(RangeKind::SignedLevel, data[0]),
            attack_time: RangedValue::from_byte(RangeKind::SignedLevel, data[1]),
            decay1_time: RangedValue::from_byte(RangeKind::SignedLevel, data[2]),
            release: RangedValue::from_byte(RangeKind::SignedLevel, data[3]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.level.as_byte(),
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
            self.release.as_byte()
        ]
    }
}

/// Velocity control.
pub struct VelocityControl {
    pub level: RangedValue,
    pub attack_time: RangedValue,
    pub decay1_time: RangedValue,
    pub release: RangedValue,
}

impl Default for VelocityControl {
    fn default() -> Self {
        VelocityControl {
            level: RangedValue::from_int(RangeKind::UnsignedLevel, 0),
            attack_time: RangedValue::from_int(RangeKind::SignedLevel, 0),
            decay1_time: RangedValue::from_int(RangeKind::SignedLevel, 0),
            release: RangedValue::from_int(RangeKind::SignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for VelocityControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        VelocityControl {
            level: RangedValue::from_byte(RangeKind::SignedLevel, data[0]),
            attack_time: RangedValue::from_byte(RangeKind::SignedLevel, data[1]),
            decay1_time: RangedValue::from_byte(RangeKind::SignedLevel, data[2]),
            release: RangedValue::from_byte(RangeKind::SignedLevel, data[3]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.level.as_byte(),
            self.attack_time.as_byte(),
            self.decay1_time.as_byte(),
            self.release.as_byte()
        ]
    }
}

/// Modulation settings for the amplifier section.
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
            ks_to_env: KeyScalingControl::from_bytes(data[..4].to_vec()),
            vel_to_env: VelocityControl::from_bytes(data[4..8].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_to_env.to_bytes());

        result
    }
}

/// Amplifier.
pub struct Amplifier {
    pub velocity_curve: RangedValue,  // 1...12 (stored as 0~11)
    pub envelope: Envelope,
    pub modulation: Modulation,
}

impl Default for Amplifier {
    fn default() -> Self {
        Amplifier {
            velocity_curve: RangedValue::from_int(RangeKind::VelocityCurve, 1),
            envelope: Default::default(),
            modulation: Default::default(),
        }
    }
}

impl SystemExclusiveData for Amplifier {
    fn from_bytes(data: Vec<u8>) -> Self {
        Amplifier {
            velocity_curve: RangedValue::from_byte(RangeKind::VelocityCurve, data[0] + 1),  // 0-11 to 1-12
            envelope: Envelope::from_bytes(data[1..7].to_vec()),
            modulation: Modulation::from_bytes(data[7..15].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.velocity_curve.as_byte() - 1);  // 1~12 to 0~11
        result.extend(self.envelope.to_bytes());
        result.extend(self.modulation.to_bytes());

        result
    }
}
