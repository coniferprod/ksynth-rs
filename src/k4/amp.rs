//! Data model for DCA.
//!

use std::convert::TryInto;
use std::fmt;
use crate::{SystemExclusiveData, ParseError};
use crate::k4::{EnvelopeTime, EnvelopeLevel, ModulationDepth, Level};

#[derive(Copy, Clone)]
pub struct Envelope {
    pub attack: EnvelopeTime,
    pub decay: EnvelopeTime,
    pub sustain: EnvelopeLevel,
    pub release: EnvelopeTime,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack: EnvelopeTime::new(54).unwrap(),
            decay: EnvelopeTime::new(72).unwrap(),
            sustain: EnvelopeLevel::new(90).unwrap(),
            release: EnvelopeTime::new(64).unwrap(),
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
        write!(f, "A={} D={} S={} R={}",
            self.attack.into_inner(),
            self.decay.into_inner(),
            self.sustain.into_inner(),
            self.release.into_inner())
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Envelope {
            attack: EnvelopeTime::new(data[0] & 0x7f).unwrap(),
            decay: EnvelopeTime::new(data[1] & 0x7f).unwrap(),
            sustain: EnvelopeLevel::new(data[2] & 0x7f).unwrap(),
            release: EnvelopeTime::new(data[3] & 0x7f).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack.into_inner(),
            self.decay.into_inner(),
            self.sustain.into_inner(),
            self.release.into_inner(),
        ]
    }

    fn data_size(&self) -> usize {
        4
    }
}

#[derive(Copy, Clone)]
pub struct LevelModulation {
    pub velocity_depth: ModulationDepth,
    pub pressure_depth: ModulationDepth,
    pub key_scaling_depth: ModulationDepth,
}

impl LevelModulation {
    pub fn new() -> LevelModulation {
        LevelModulation {
            velocity_depth: ModulationDepth::new(15).unwrap(),
            pressure_depth: ModulationDepth::new(0).unwrap(),
            key_scaling_depth: ModulationDepth::new(-6).unwrap(),
        }
    }
}

impl Default for LevelModulation {
    fn default() -> Self {
        LevelModulation::new()
    }
}

impl fmt::Display for LevelModulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vel.depth={} Prs.depth={} KS depth={}",
            self.velocity_depth.into_inner(),
            self.pressure_depth.into_inner(),
            self.key_scaling_depth.into_inner()
        )
    }
}

impl SystemExclusiveData for LevelModulation {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(LevelModulation {
            velocity_depth: ModulationDepth::new((data[0] as i8) - 50).unwrap(),
            pressure_depth: ModulationDepth::new((data[1] as i8) - 50).unwrap(),
            key_scaling_depth: ModulationDepth::new((data[2] as i8) - 50).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            (self.velocity_depth.into_inner() + 50).try_into().unwrap(),
            (self.pressure_depth.into_inner() + 50).try_into().unwrap(),
            (self.key_scaling_depth.into_inner() + 50).try_into().unwrap(),
        ]
    }

    fn data_size(&self) -> usize {
        3
    }
}

#[derive(Copy, Clone)]
pub struct TimeModulation {
    pub attack_velocity: ModulationDepth,
    pub release_velocity: ModulationDepth,
    pub key_scaling: ModulationDepth,
}

impl TimeModulation {
    pub fn new() -> TimeModulation {
        TimeModulation {
            attack_velocity: ModulationDepth::new(0).unwrap(),
            release_velocity: ModulationDepth::new(0).unwrap(),
            key_scaling: ModulationDepth::new(0).unwrap(),
        }
    }
}

impl Default for TimeModulation {
    fn default() -> Self {
        TimeModulation::new()
    }
}

impl fmt::Display for TimeModulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Atk.vel={} Rel.vel={} KS={}",
            self.attack_velocity.into_inner(),
            self.release_velocity.into_inner(),
            self.key_scaling.into_inner()
        )
    }
}

impl SystemExclusiveData for TimeModulation {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(TimeModulation {
            attack_velocity: ModulationDepth::new((data[0] as i8) - 50).unwrap(),
            release_velocity: ModulationDepth::new((data[1] as i8) - 50).unwrap(),
            key_scaling: ModulationDepth::new((data[2] as i8) - 50).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            (self.attack_velocity.into_inner() + 50).try_into().unwrap(),
            (self.release_velocity.into_inner() + 50).try_into().unwrap(),
            (self.key_scaling.into_inner() + 50).try_into().unwrap(),
        ]
    }

    fn data_size(&self) -> usize {
        3
    }
}

#[derive(Copy, Clone)]
pub struct Amplifier {
    pub level: Level,
    pub envelope: Envelope,
    pub level_modulation: LevelModulation,
    pub time_modulation: TimeModulation,
}

impl Amplifier {
    pub fn new() -> Amplifier {
        Amplifier {
            level: Level::new(75).unwrap(),
            envelope: Default::default(),
            level_modulation: Default::default(),
            time_modulation: Default::default(),
        }
    }
}

impl Default for Amplifier {
    fn default() -> Self {
        Amplifier::new()
    }
}

impl fmt::Display for Amplifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Level={} Envelope={} LevelMod={} TimeMod={}",
            self.level.into_inner(), self.envelope, self.level_modulation, self.time_modulation
        )
    }
}

impl SystemExclusiveData for Amplifier {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset: usize = 0;
        let mut start: usize;
        let mut end: usize;

        let b = data[offset];
        offset += 1;
        let level = Level::new(b & 0x7f).unwrap();

        start = offset;
        end = start + 4;
        let envelope_bytes = &data[start..end];
        let envelope = Envelope::from_bytes(&envelope_bytes);
        offset += 4;

        start = offset;
        end = start + 3;
        let level_mod_bytes = &data[start..end];
        let level_modulation = LevelModulation::from_bytes(&level_mod_bytes);
        offset += 3;

        start = offset;
        end = start + 3;
        let time_mod_bytes = &data[start..end];
        let time_modulation = TimeModulation::from_bytes(&time_mod_bytes);

        Ok(Amplifier {
            level,
            envelope: envelope?,
            level_modulation: level_modulation?,
            time_modulation: time_modulation?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.level.into_inner());
        buf.extend(self.envelope.to_bytes());
        buf.extend(self.level_modulation.to_bytes());
        buf.extend(self.time_modulation.to_bytes());

        buf
    }

    fn data_size(&self) -> usize {
        1
            + self.envelope.data_size()
            + self.level_modulation.data_size()
            + self.time_modulation.data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_amplifier_envelope() {
        let env = Envelope {
            attack: EnvelopeTime::new(10).unwrap(),
            decay: EnvelopeTime::new(5).unwrap(),
            sustain: EnvelopeLevel::new(20).unwrap(),
            release: EnvelopeTime::new(10).unwrap(),
        };

        assert_eq!(
            vec![
                env.attack.into_inner(),
                env.decay.into_inner(),
                env.sustain.into_inner(),
                env.release.into_inner()
            ],
            vec![
                10,
                5,
                20,
                10
            ]
        )
    }
}
