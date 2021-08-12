//! Data model for the pitch envelope.
//!

use crate::SystemExclusiveData;
use crate::k5000::{RangedValue, RangeKind};

/// Pitch envelope.
pub struct Envelope {
    pub start: RangedValue,
    pub attack_time: RangedValue,
    pub attack_level: RangedValue,
    pub decay_time: RangedValue,
    pub time_vel_sens: RangedValue,
    pub level_vel_sens: RangedValue,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            start: RangedValue::from_int(RangeKind::SignedLevel, 0),
            attack_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            attack_level: RangedValue::from_int(RangeKind::SignedLevel, 0),
            decay_time: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            time_vel_sens: RangedValue::from_int(RangeKind::SignedLevel, 0),
            level_vel_sens: RangedValue::from_int(RangeKind::SignedLevel, 0),
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            start: RangedValue::from_byte(RangeKind::SignedLevel, data[0]),
            attack_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[1]),
            attack_level: RangedValue::from_byte(RangeKind::SignedLevel, data[2]),
            decay_time: RangedValue::from_byte(RangeKind::PositiveLevel, data[3]),
            time_vel_sens: RangedValue::from_byte(RangeKind::SignedLevel, data[4]),
            level_vel_sens: RangedValue::from_byte(RangeKind::SignedLevel, data[5]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let bs = vec![
            self.start.as_byte(),
            self.attack_time.as_byte(),
            self.attack_level.as_byte(),
            self.decay_time.as_byte(),
            self.time_vel_sens.as_byte(),
            self.level_vel_sens.as_byte()
        ];
        result.extend(bs);
        result
    }
}
