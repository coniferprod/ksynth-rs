//! Data model for the amplifier (DCA).
//!

use std::convert::TryFrom;
use std::fmt;

use crate::{SystemExclusiveData, ParseError};
use crate::k5000::{EnvelopeTime, EnvelopeLevel, ControlTime, KeyScaling, VelocityControlLevel};
use crate::k5000::control::VelocityCurve;

/// Amplifier envelope.
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
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
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
            self.release_time.into()
        ]
    }
}

/// Amplifier key scaling control.
#[derive(Debug)]
pub struct KeyScalingControl {
    pub level: KeyScaling,
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
    pub release: ControlTime,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        KeyScalingControl {
            level: KeyScaling::new(0),
            attack_time: ControlTime::new(0),
            decay1_time: ControlTime::new(0),
            release: ControlTime::new(0),
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
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(KeyScalingControl {
            level: KeyScaling::from(data[0]),
            attack_time: ControlTime::from(data[1]),
            decay1_time: ControlTime::from(data[2]),
            release: ControlTime::from(data[3]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.level.into(),
            self.attack_time.into(),
            self.decay1_time.into(),
            self.release.into()
        ]
    }
}

/// Amplifier velocity control.
#[derive(Debug)]
pub struct VelocityControl {
    pub level: VelocityControlLevel,
    pub attack_time: ControlTime,
    pub decay1_time: ControlTime,
    pub release: ControlTime,
}

impl Default for VelocityControl {
    fn default() -> Self {
        VelocityControl {
            level: VelocityControlLevel::new(0),
            attack_time: ControlTime::new(0),
            decay1_time: ControlTime::new(0),
            release: ControlTime::new(0),
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
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(VelocityControl {
            level: VelocityControlLevel::from(data[0]),
            attack_time: ControlTime::from(data[1]),
            decay1_time: ControlTime::from(data[2]),
            release: ControlTime::from(data[3]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.level.into(),
            self.attack_time.into(),
            self.decay1_time.into(),
            self.release.into()
        ]
    }
}

/// Modulation settings for the amplifier section.
#[derive(Debug)]
pub struct Modulation {
    pub ks_to_env: KeyScalingControl,
    pub vel_sens: VelocityControl,
}

impl Default for Modulation {
    fn default() -> Self {
        Modulation {
            ks_to_env: Default::default(),
            vel_sens: Default::default(),
        }
    }
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KS to Env.: {}\nVel.sens.: {}",
            self.ks_to_env, self.vel_sens
        )
    }
}

impl SystemExclusiveData for Modulation {
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(Modulation {
            ks_to_env: KeyScalingControl::from_bytes(data[..4].to_vec())?,
            vel_sens: VelocityControl::from_bytes(data[4..8].to_vec())?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_sens.to_bytes());

        result
    }
}

/// Amplifier.
#[derive(Debug)]
pub struct Amplifier {
    pub velocity_curve: VelocityCurve,  // 1...12 (stored as 0~11)
    pub envelope: Envelope,
    pub modulation: Modulation,
}

impl Default for Amplifier {
    fn default() -> Self {
        Amplifier {
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
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(Amplifier {
            velocity_curve: VelocityCurve::try_from(data[0]).unwrap(),  // 0-11 to enum
            envelope: Envelope::from_bytes(data[1..7].to_vec())?,
            modulation: Modulation::from_bytes(data[7..15].to_vec())?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.velocity_curve as u8);  // raw enum values map to 0~11
        result.extend(self.envelope.to_bytes());
        result.extend(self.modulation.to_bytes());

        result
    }
}
