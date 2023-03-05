use std::convert::TryInto;
use std::fmt;

use bit::BitIndex;

use crate::SystemExclusiveData;
use crate::k4::{EnvelopeTime, FilterEnvelopeLevel, Cutoff, Resonance, ModulationDepth};
use crate::k4::amp::{LevelModulation, TimeModulation};

#[derive(Copy, Clone)]
pub struct Envelope {
    pub attack: EnvelopeTime,
    pub decay: EnvelopeTime,
    pub sustain: FilterEnvelopeLevel,
    pub release: EnvelopeTime,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            attack: EnvelopeTime::new(0).unwrap(),
            decay: EnvelopeTime::new(50).unwrap(),
            sustain: FilterEnvelopeLevel::new(25).unwrap(),
            release: EnvelopeTime::new(25).unwrap(),
        }
    }
}

impl Envelope {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "A={} D={} S={} R={}",
            self.attack.into_inner(),
            self.decay.into_inner(),
            self.sustain.into_inner(),
            self.release.into_inner()
        )
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            attack: EnvelopeTime::new(data[0]).unwrap(),
            decay: EnvelopeTime::new(data[1]).unwrap(),
            sustain: FilterEnvelopeLevel::new((data[2] as i8) - 50).unwrap(),
            release: EnvelopeTime::new(data[3]).unwrap(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.attack.into_inner());
        buf.push(self.decay.into_inner());
        buf.push((self.sustain.into_inner() + 50).try_into().unwrap());
        buf.push(self.release.into_inner());

        buf
    }

    fn data_size(&self) -> usize { 4 }
}

#[derive(Copy, Clone)]
pub struct Filter {
    pub cutoff: Cutoff,  // 0~100
    pub resonance: Resonance,  // 0~7
    pub cutoff_mod: LevelModulation,
    pub lfo_modulates_cutoff: bool,
    pub envelope: Envelope,
    pub env_depth: ModulationDepth,
    pub env_vel_depth: ModulationDepth,
    pub time_mod: TimeModulation,
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            cutoff: Cutoff::new(49).unwrap(),
            resonance: Resonance::new(2).unwrap(),
            cutoff_mod: Default::default(),
            lfo_modulates_cutoff: false,
            env_depth: ModulationDepth::new(0).unwrap(),
            env_vel_depth: ModulationDepth::new(0).unwrap(),
            envelope: Default::default(),
            time_mod: Default::default(),
        }
    }
}

impl Filter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "cutoff = {}, resonance = {}, LFO sw = {}, cutoff mod = {}, env = {}, env depth = {}, env vel.depth = {}",
            self.cutoff.into_inner(),
            self.resonance.into_inner(),
            self.lfo_modulates_cutoff,
            self.cutoff_mod,
            self.envelope,
            self.env_depth.into_inner(),
            self.env_vel_depth.into_inner()
        )
    }
}

impl SystemExclusiveData for Filter {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;
        let mut start: usize;
        let mut end: usize;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let cutoff = b & 0x7f;

        b = data[offset];
        offset += 1;
        let resonance = (b & 0x07) + 1;  // from 0...7 to 1...8
        let lfo_modulates_cutoff = b.bit(3);

        start = offset;
        end = start + 3;
        let cutoff_mod_bytes = data[start..end].to_vec();
        let cutoff_mod = LevelModulation::from_bytes(cutoff_mod_bytes);

        offset += 3;
        b = data[offset];
        offset += 1;
        let env_depth = (b as i8) - 50;

        b = data[offset];
        offset += 1;
        let env_vel_depth = (b as i8) - 50;

        start = offset;
        end = start + 4;
        let envelope_bytes = data[start..end].to_vec();
        let envelope = Envelope::from_bytes(envelope_bytes);
        offset += 4;

        start = offset;
        end = start + 3;
        let time_mod_bytes = data[start..end].to_vec();
        let time_mod = TimeModulation::from_bytes(time_mod_bytes);

        Filter {
            cutoff: Cutoff::new(cutoff).unwrap(),
            resonance: Resonance::new(resonance).unwrap(),
            cutoff_mod,
            lfo_modulates_cutoff,
            env_depth: ModulationDepth::new(env_depth).unwrap(),
            env_vel_depth: ModulationDepth::new(env_vel_depth).unwrap(),
            envelope,
            time_mod,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.cutoff.into_inner());
        let mut s104 = self.resonance.into_inner();
        if self.lfo_modulates_cutoff {
            s104.set_bit(3, true);
        }
        buf.push(s104);

        buf.extend(self.cutoff_mod.to_bytes());
        buf.push((self.env_depth.into_inner() + 50).try_into().unwrap());
        buf.push((self.env_vel_depth.into_inner() + 50).try_into().unwrap());
        buf.extend(self.envelope.to_bytes());
        buf.extend(self.time_mod.to_bytes());

        buf
    }

    fn data_size(&self) -> usize {
        2
            + self.cutoff_mod.data_size()
            + 2
            + self.envelope.data_size()
            + self.time_mod.data_size()
    }
}
