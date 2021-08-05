use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
/// Filter mode.
pub enum FilterMode {
    LowPass = 0,
    HighPass = 1,
}

#[derive(Debug)]
/// Filter envelope.
pub struct Envelope {
    pub attack_time: u32,
    pub decay1_time: u32,
    pub decay1_level: i32,
    pub decay2_time: u32,
    pub decay2_level: i32,
    pub release_time: u32,
    pub ks_to_attack: i32,
    pub ks_to_decay1: i32,
    pub vel_to_envelope: i32,
    pub vel_to_attack: i32,
    pub vel_to_decay1: i32,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack_time: 0,
            decay1_time: 0,
            decay1_level: 0,
            decay2_time: 0,
            decay2_level: 0,
            release_time: 0,
            ks_to_attack: 0,
            ks_to_decay1: 0,
            vel_to_envelope: 0,
            vel_to_attack: 0,
            vel_to_decay1: 0,
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
            attack_time: data[0] as u32,
            decay1_time: data[1] as u32,
            decay1_level: (data[2] - 64) as i32,
            decay2_time: data[3] as u32,
            decay2_level: (data[4] - 64) as i32,
            release_time: data[5] as u32,
            ks_to_attack: (data[6] - 64) as i32,
            ks_to_decay1: (data[7] - 64) as i32,
            vel_to_envelope: (data[8] - 64) as i32,
            vel_to_attack: (data[9] - 64) as i32,
            vel_to_decay1: (data[10] - 64) as i32,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            self.attack_time as u8,
            self.decay1_time as u8,
            (self.decay1_level + 64) as u8,
            self.decay2_time as u8,
            (self.decay2_level + 64) as u8,
            self.release_time as u8,
            (self.ks_to_attack + 64) as u8,
            (self.ks_to_decay1 + 64) as u8,
            (self.vel_to_envelope + 64) as u8,
            (self.vel_to_attack + 64) as u8,
            (self.vel_to_decay1 + 64) as u8,
        ];
        result.extend(bs);

        result
    }
}

/// Filter settings.
pub struct Filter {
    pub is_active: bool,
    pub cutoff: u32,
    pub resonance: u32,
    pub mode: FilterMode,
    pub velocity_curve: u32,
    pub level: u32,
    pub ks_to_cutoff: i32,
    pub vel_to_cutoff: i32,
    pub envelope_depth: i32,
    pub envelope: Envelope,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            is_active: true,
            cutoff: 0,
            resonance: 0,
            mode: FilterMode::LowPass,
            velocity_curve: 1,
            level: 0,
            ks_to_cutoff: 0,
            vel_to_cutoff: 0,
            envelope_depth: 0,
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
            velocity_curve: data[2] as u32 + 1,  // adjust from 0 ~ 11 to 1 ~ 12
            resonance: data[3] as u32,
            level: data[4] as u32,
            cutoff: data[5] as u32,
            ks_to_cutoff: data[6] as i32 - 64,
            vel_to_cutoff: data[7] as i32 - 64,
            envelope_depth: data[8] as i32 - 64,
            envelope: Envelope::from_bytes(data[9..].to_vec())
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bs = vec![
            if self.is_active { 0 } else { 1 },  // is this the right way around?
            self.mode as u8,
            self.velocity_curve as u8 - 1,
            self.resonance as u8,
            self.level as u8,
            self.cutoff as u8,
            (self.ks_to_cutoff + 64) as u8,
            (self.vel_to_cutoff + 64) as u8,
            (self.envelope_depth + 64) as u8,
        ];
        result.extend(bs);
        result.extend(self.envelope.to_bytes());

        result
    }
}
