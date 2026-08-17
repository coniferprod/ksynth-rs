//! Data model for DCA.
//!

use std::convert::TryInto;
use std::fmt;

use syxpack::{
    Ranged, 
    SystemExclusiveData, 
    ParseError,
    parse_or_default,
    Encoding,
};

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
            attack: EnvelopeTime::new(54),
            decay: EnvelopeTime::new(72),
            sustain: EnvelopeLevel::new(90),
            release: EnvelopeTime::new(64),
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
            self.attack,
            self.decay,
            self.sustain,
            self.release
        )
    }
}

impl SystemExclusiveData for Envelope {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Envelope {
            attack: parse_or_default::<EnvelopeTime>(data[0] & 0x7f),
            decay: parse_or_default::<EnvelopeTime>(data[1] & 0x7f),
            sustain: parse_or_default::<EnvelopeLevel>(data[2] & 0x7f),
            release: parse_or_default::<EnvelopeTime>(data[3] & 0x7f),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack.encode(),
            self.decay.encode(),
            self.sustain.encode(),
            self.release.encode(),
        ]
    }

    fn data_size() -> usize { 4 }
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
            velocity_depth: ModulationDepth::new(15),
            pressure_depth: ModulationDepth::new(0),
            key_scaling_depth: ModulationDepth::new(-6),
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
            self.velocity_depth,
            self.pressure_depth,
            self.key_scaling_depth
        )
    }
}

impl SystemExclusiveData for LevelModulation {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(LevelModulation {
            velocity_depth: parse_or_default::<ModulationDepth>(data[0]),
            pressure_depth: parse_or_default::<ModulationDepth>(data[1]),
            key_scaling_depth: parse_or_default::<ModulationDepth>(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.velocity_depth.encode(),
            self.pressure_depth.encode(),
            self.key_scaling_depth.encode(),
        ]
    }

    fn data_size() -> usize { 3 }
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
            attack_velocity: ModulationDepth::new(0),
            release_velocity: ModulationDepth::new(0),
            key_scaling: ModulationDepth::new(0),
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
            self.attack_velocity,
            self.release_velocity,
            self.key_scaling
        )
    }
}

impl SystemExclusiveData for TimeModulation {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(TimeModulation {
            attack_velocity: parse_or_default::<ModulationDepth>(data[0]),
            release_velocity: parse_or_default::<ModulationDepth>(data[1]),
            key_scaling: parse_or_default::<ModulationDepth>(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack_velocity.encode(),
            self.release_velocity.encode(),
            self.key_scaling.encode(),
        ]
    }

    fn data_size() -> usize { 3 }
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
            level: Level::new(75),
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
            self.level, self.envelope, self.level_modulation, self.time_modulation
        )
    }
}

impl SystemExclusiveData for Amplifier {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset: usize = 0;
        let mut start: usize;
        let mut end: usize;

        let b = data[offset];
        offset += 1;
        let level = parse_or_default::<Level>(b & 0x7f);

        start = offset;
        end = start + 4;
        let envelope_bytes = &data[start..end];
        let envelope = Envelope::parse(&envelope_bytes);
        offset += 4;

        start = offset;
        end = start + 3;
        let level_mod_bytes = &data[start..end];
        let level_modulation = LevelModulation::parse(&level_mod_bytes);
        offset += 3;

        start = offset;
        end = start + 3;
        let time_mod_bytes = &data[start..end];
        let time_modulation = TimeModulation::parse(&time_mod_bytes);

        Ok(Amplifier {
            level,
            envelope: envelope?,
            level_modulation: level_modulation?,
            time_modulation: time_modulation?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.level.encode());
        buf.extend(self.envelope.to_bytes());
        buf.extend(self.level_modulation.to_bytes());
        buf.extend(self.time_modulation.to_bytes());

        buf
    }

    fn data_size() -> usize {
        1
        + Envelope::data_size()
        + LevelModulation::data_size()
        + TimeModulation::data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_amplifier_envelope() {
        let env = Envelope {
            attack: EnvelopeTime::new(10),
            decay: EnvelopeTime::new(5),
            sustain: EnvelopeLevel::new(20),
            release: EnvelopeTime::new(10),
        };

        assert_eq!(
            vec![
                env.attack.value(),
                env.decay.value(),
                env.sustain.value(),
                env.release.value()
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
