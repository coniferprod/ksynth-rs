//! Data model for the amplifier (DCA).
//!

use std::convert::TryFrom;
use std::fmt;

use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{
    SystemExclusiveData,
    ParseError,
    Ranged, ranged_impl,
};
use crate::k5000::{DESCRIPTORS, ByteValue};
use crate::k5000::control::VelocityCurve;

// Amplifier envelope level is different from the other
// envelope levels; it goes from 0 to 127, while the
// others are -63 to 63. So we define our own type in
// this module, and *don't* import the normal level type.

/// Amplifier envelope level (0...127, default 0)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EnvelopeLevel(i32);
ranged_impl!(EnvelopeLevel, 0, 127, 0);

impl From<u8> for EnvelopeLevel {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl From<EnvelopeLevel> for u8 {
    fn from(value: EnvelopeLevel) -> Self {
        value.value() as u8
    }
}

/// Amplifier envelope.
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
        Envelope {
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
        let level_desc = DESCRIPTORS.get(&ByteValue::AmplifierEnvelopeLevel).unwrap();

        Ok(Envelope {
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
        let level_desc = DESCRIPTORS.get(&ByteValue::AmplifierEnvelopeLevel).unwrap();

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

/// Amplifier key scaling control.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyScalingControl {
    pub level: i32, // KeyScaling,
    pub attack_time: i32, // ControlTime,
    pub decay1_time: i32, // ControlTime,
    pub release: i32, // ControlTime,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        KeyScalingControl {
            level: Default::default(),
            attack_time: Default::default(),
            decay1_time: Default::default(),
            release: Default::default(),
        }
    }
}

impl fmt::Display for KeyScalingControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Level={}  Attack={} Decay1={} Release={}",
            self.level, self.attack_time, self.decay1_time, self.release
        )
    }
}

impl SystemExclusiveData for KeyScalingControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScaling).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        Ok(Self {
            level: (ks_desc.incoming)(data[0]),
            attack_time: (ct_desc.incoming)(data[1]),
            decay1_time: (ct_desc.incoming)(data[2]),
            release: (ct_desc.incoming)(data[3]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let ks_desc = DESCRIPTORS.get(&ByteValue::KeyScaling).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        vec![
            (ks_desc.outgoing)(self.level),
            (ct_desc.outgoing)(self.attack_time),
            (ct_desc.outgoing)(self.decay1_time),
            (ct_desc.outgoing)(self.release),
        ]
    }

    fn data_size() -> usize { 4 }
}

/// Amplifier velocity control.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VelocityControl {
    pub level: i32, // VelocityControlLevel,
    pub attack_time: i32, // ControlTime,
    pub decay1_time: i32, // ControlTime,
    pub release: i32, // ControlTime,
}

impl Default for VelocityControl {
    fn default() -> Self {
        Self {
            level: Default::default(),
            attack_time: Default::default(),
            decay1_time: Default::default(),
            release: Default::default(),
        }
    }
}

impl fmt::Display for VelocityControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Level={} Attack={} Decay1={} Release={}",
            self.level, self.attack_time, self.decay1_time, self.release
        )
    }
}

impl SystemExclusiveData for VelocityControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let vcl_desc = DESCRIPTORS.get(&ByteValue::VelocityControlLevel).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        Ok(Self {
            level: (vcl_desc.incoming)(data[0]),
            attack_time: (ct_desc.incoming)(data[1]),
            decay1_time: (ct_desc.incoming)(data[2]),
            release: (ct_desc.incoming)(data[3]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let vcl_desc = DESCRIPTORS.get(&ByteValue::VelocityControlLevel).unwrap();
        let ct_desc = DESCRIPTORS.get(&ByteValue::ControlTime).unwrap();

        vec![
            (vcl_desc.outgoing)(self.level),
            (ct_desc.outgoing)(self.attack_time),
            (ct_desc.outgoing)(self.decay1_time),
            (ct_desc.outgoing)(self.release),
        ]
    }

    fn data_size() -> usize { 4 }
}

/// Modulation settings for the amplifier section.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modulation {
    pub ks_to_env: KeyScalingControl,
    pub vel_sens: VelocityControl,
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KS to Env.: {}\nVel.sens.: {}",
            self.ks_to_env, self.vel_sens
        )
    }
}

impl SystemExclusiveData for Modulation {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Modulation {
            ks_to_env: KeyScalingControl::from_bytes(&data[..4])?,
            vel_sens: VelocityControl::from_bytes(&data[4..8])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_sens.to_bytes());

        result
    }

    fn data_size() -> usize {
        KeyScalingControl::data_size()
        + VelocityControl::data_size()
    }
}

/// Amplifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amplifier {
    pub velocity_curve: VelocityCurve,  // 1...12 (stored as 0~11)
    pub envelope: Envelope,
    pub modulation: Modulation,
}

impl Default for Amplifier {
    fn default() -> Self {
        Self {
            velocity_curve: VelocityCurve::Curve1,
            envelope: Default::default(),
            modulation: Default::default(),
        }
    }
}

impl fmt::Display for Amplifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vel. curve: {}\nEnvelope: {}\nModulation: {}",
            self.velocity_curve, self.envelope, self.modulation
        )
    }
}

impl SystemExclusiveData for Amplifier {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            velocity_curve: VelocityCurve::try_from(data[0]).unwrap(),  // 0-11 to enum
            envelope: Envelope::from_bytes(&data[1..7])?,
            modulation: Modulation::from_bytes(&data[7..15])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.velocity_curve as u8);  // raw enum values map to 0~11
        result.extend(self.envelope.to_bytes());
        result.extend(self.modulation.to_bytes());

        result
    }

    fn data_size() -> usize {
        1  // velocity curve
        + Envelope::data_size()
        + Modulation::data_size()
    }
}
