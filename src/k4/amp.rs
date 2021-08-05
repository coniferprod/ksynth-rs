use std::convert::TryInto;
use std::fmt;

pub struct AmplifierEnvelope {
    pub attack: u8,
    pub decay: u8,
    pub sustain: u8,
    pub release: u8,
}

impl AmplifierEnvelope {
    pub fn new() -> AmplifierEnvelope {
        AmplifierEnvelope {
            attack: 54,
            decay: 72,
            sustain: 90,
            release: 64,
        }
    }
}

impl Default for AmplifierEnvelope {
    fn default() -> Self {
        AmplifierEnvelope::new()
    }
}

impl fmt::Display for AmplifierEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A={} D={} S={} R={}", self.attack, self.decay, self.sustain, self.release)
    }
}

impl crate::SystemExclusiveData for AmplifierEnvelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        AmplifierEnvelope {
            attack: data[0] & 0x7f,
            decay: data[1] & 0x7f,
            sustain: data[2] & 0x7f,
            release: data[3] & 0x7f,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack,
            self.decay,
            self.sustain,
            self.release
        ]
    }

    /*
    fn data_size(&self) -> usize {
        4
    }
    */
}

pub struct LevelModulation {
    pub velocity_depth: i8,
    pub pressure_depth: i8,
    pub key_scaling_depth: i8,
}

impl LevelModulation {
    pub fn new() -> LevelModulation {
        LevelModulation {
            velocity_depth: 15,
            pressure_depth: 0,
            key_scaling_depth: -6,
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
            self.velocity_depth, self.pressure_depth, self.key_scaling_depth
        )
    }
}

impl crate::SystemExclusiveData for LevelModulation {
    fn from_bytes(data: Vec<u8>) -> Self {
        LevelModulation {
            velocity_depth: (data[0] as i8) - 50,
            pressure_depth: (data[1] as i8) - 50,
            key_scaling_depth: (data[2] as i8) - 50,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            (self.velocity_depth + 50).try_into().unwrap(),
            (self.pressure_depth + 50).try_into().unwrap(),
            (self.key_scaling_depth + 50).try_into().unwrap(),
        ]
    }

    /*
    fn data_size(&self) -> usize {
        3
    }
    */
}

pub struct TimeModulation {
    pub attack_velocity: i8,
    pub release_velocity: i8,
    pub key_scaling: i8,
}

impl TimeModulation {
    pub fn new() -> TimeModulation {
        TimeModulation {
            attack_velocity: 0,
            release_velocity: 0,
            key_scaling: 0,
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
            self.attack_velocity, self.release_velocity, self.key_scaling
        )
    }
}

impl crate::SystemExclusiveData for TimeModulation {
    fn from_bytes(data: Vec<u8>) -> Self {
        TimeModulation {
            attack_velocity: (data[0] as i8) - 50,
            release_velocity: (data[1] as i8) - 50,
            key_scaling: (data[2] as i8) - 50,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            (self.attack_velocity + 50).try_into().unwrap(),
            (self.release_velocity + 50).try_into().unwrap(),
            (self.key_scaling + 50).try_into().unwrap(),
        ]
    }

    /*
    fn data_size(&self) -> usize {
        3
    }
    */
}

pub struct Amplifier {
    pub level: u8,
    pub envelope: AmplifierEnvelope,
    pub level_modulation: LevelModulation,
    pub time_modulation: TimeModulation,
}

impl Amplifier {
    pub fn new() -> Amplifier {
        Amplifier {
            level: 75,
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

impl crate::SystemExclusiveData for Amplifier {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let level = b & 0x7f;

        start = offset;
        end = start + 4;
        let envelope_bytes = data[start..end].to_vec();
        let envelope = crate::k4::amp::AmplifierEnvelope::from_bytes(envelope_bytes);
        offset += 4;

        start = offset;
        end = start + 3;
        let level_mod_bytes = data[start..end].to_vec();
        let level_modulation = crate::k4::amp::LevelModulation::from_bytes(level_mod_bytes);
        offset += 3;

        start = offset;
        end = start + 3;
        let time_mod_bytes = data[start..end].to_vec();
        let time_modulation = crate::k4::amp::TimeModulation::from_bytes(time_mod_bytes);

        Amplifier {
            level,
            envelope,
            level_modulation,
            time_modulation,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.level);
        buf.extend(self.envelope.to_bytes());
        buf.extend(self.level_modulation.to_bytes());
        buf.extend(self.time_modulation.to_bytes());

        buf
    }

    /*
    fn data_size(&self) -> usize {
        1
            + self.envelope.data_size()
            + self.level_modulation.data_size()
            + self.time_modulation.data_size()
    }
    */
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_amplifier_envelope() {
        let env = AmplifierEnvelope {
            attack: 10,
            decay: 5,
            sustain: 20,
            release: 10,
        };

        assert_eq!(vec![env.attack, env.decay, env.sustain, env.release], vec![10, 5, 20, 10])
    }
}
