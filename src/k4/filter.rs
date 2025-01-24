//! Data model for DCF.
//!

use std::fmt;

use bit::BitIndex;

use crate::{
    SystemExclusiveData,
    ParseError,
    Ranged
};
use crate::k4::{
    EnvelopeTime,
    FilterEnvelopeLevel,
    Cutoff,
    Resonance,
    ModulationDepth
};
use crate::k4::amp::{
    LevelModulation,
    TimeModulation
};

/// Filter envelope.
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
            attack: EnvelopeTime::new(0),
            decay: EnvelopeTime::new(50),
            sustain: FilterEnvelopeLevel::new(25),
            release: EnvelopeTime::new(25),
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
            self.attack.value(),
            self.decay.value(),
            self.sustain.value(),
            self.release.value()
        )
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Envelope {
            attack: EnvelopeTime::new(data[0] as i32),
            decay: EnvelopeTime::new(data[1] as i32),
            sustain: FilterEnvelopeLevel::new((data[2] as i32) - 50),
            release: EnvelopeTime::new(data[3] as i32),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.attack.value() as u8,
            self.decay.value() as u8,
            (self.sustain.value() + 50) as u8,
            self.release.value() as u8,
        ]
    }

    fn data_size() -> usize { 4 }
}

/// Filter (DCF).
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
            cutoff: Cutoff::new(49),
            resonance: Resonance::new(2),
            cutoff_mod: Default::default(),
            lfo_modulates_cutoff: false,
            env_depth: ModulationDepth::new(0),
            env_vel_depth: ModulationDepth::new(0),
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
            self.cutoff.value(),
            self.resonance.value(),
            self.lfo_modulates_cutoff,
            self.cutoff_mod,
            self.envelope,
            self.env_depth.value(),
            self.env_vel_depth.value()
        )
    }
}

impl SystemExclusiveData for Filter {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset: usize = 0;
        let mut start: usize;
        let mut end: usize;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let cutoff = b & 0x7f;

        b = data[offset];
        offset += 1;

        // The Kawai K4 SysEx spec has "0~7 / 1~8" for resonance,
        // but it's described as 0...7 in the user manual, so let's go with that.
        let resonance = b & 0b111;  // use the bottom three bits
        let lfo_modulates_cutoff = b.bit(3);

        start = offset;
        end = start + 3;
        let cutoff_mod_bytes = &data[start..end];
        let cutoff_mod = LevelModulation::from_bytes(&cutoff_mod_bytes);

        offset += 3;
        b = data[offset];
        offset += 1;
        let env_depth = (b as i8) - 50;

        b = data[offset];
        offset += 1;
        let env_vel_depth = (b as i8) - 50;

        start = offset;
        end = start + 4;
        let envelope_bytes = &data[start..end];
        let envelope = Envelope::from_bytes(&envelope_bytes);
        offset += 4;

        start = offset;
        end = start + 3;
        let time_mod_bytes = &data[start..end];
        let time_mod = TimeModulation::from_bytes(&time_mod_bytes);

        Ok(Filter {
            cutoff: Cutoff::new(cutoff as i32),
            resonance: Resonance::new(resonance as i32),
            cutoff_mod: cutoff_mod?,
            lfo_modulates_cutoff,
            env_depth: ModulationDepth::new(env_depth as i32),
            env_vel_depth: ModulationDepth::new(env_vel_depth as i32),
            envelope: envelope?,
            time_mod: time_mod?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.cutoff.value() as u8);
        let mut s104 = self.resonance.value() as u8;
        if self.lfo_modulates_cutoff {
            s104.set_bit(3, true);
        }
        buf.push(s104);

        buf.extend(self.cutoff_mod.to_bytes());
        buf.push((self.env_depth.value() + 50) as u8);
        buf.push((self.env_vel_depth.value() + 50) as u8);
        buf.extend(self.envelope.to_bytes());
        buf.extend(self.time_mod.to_bytes());

        buf
    }

    fn data_size() -> usize {
        2
        + LevelModulation::data_size()
        + 2
        + Envelope::data_size()
        + TimeModulation::data_size()
    }
}
