use std::convert::TryFrom;
use std::convert::TryInto;
use num_enum::TryFromPrimitive;
use std::fmt;
use crate::k4::SUBMIX_COUNT;
use crate::{SystemExclusiveData, Checksum};

static EFFECT_NAMES: &[&str] = &[
    "None",  // just to align with 1...16
    "Reverb 1",
    "Reverb 2",
    "Reverb 3",
    "Reverb 4",
    "Gate Reverb",
    "Reverse Gate",
    "Normal Delay",
    "Stereo Panpot Delay",
    "Chorus",
    "Overdrive + Flanger",
    "Overdrive + Normal Delay",
    "Overdrive + Reverb",
    "Normal Delay + Normal Delay",
    "Normal Delay + Stereo Panpot Delay",
    "Chorus + Normal Delay",
    "Chorus + Stereo Panpot Delay",
];

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Effect {
    None,  // not really used, just to align the names
    Reverb1,
    Reverb2,
    Reverb3,
    Reverb4,
    GateReverb,
    ReverseGate,
    NormalDelay,
    StereoPanpotDelay,
    Chorus,
    OverdrivePlusFlanger,
    OverdrivePlusNormalDelay,
    OverdrivePlusReverb,
    NormalDelayPlusNormalDelay,
    NormalDelayPlusStereoPanpotDelay,
    ChorusPlusNormalDelay,
    ChorusPlusStereoPanpotDelay,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", EFFECT_NAMES[*self as usize].to_string())
    }
}

#[derive(Clone)]
pub struct EffectPatch {
    pub effect: Effect,
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
    pub submixes: [SubmixSettings; SUBMIX_COUNT],
}

impl Default for EffectPatch {
    fn default() -> Self {
        EffectPatch {
            effect: Effect::Reverb1,
            param1: 0,
            param2: 0,
            param3: 0,
            submixes: [Default::default(); SUBMIX_COUNT],
        }
    }
}

impl fmt::Display for EffectPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "effect = {}, param1 = {}, param2 = {}, param3 = {}",
            EFFECT_NAMES[self.effect as usize].to_string(), self.param1, self.param2, self.param3
        )
    }
}

impl EffectPatch {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.effect as u8 - 1);
        buf.push(self.param1);
        buf.push(self.param2);
        buf.push(self.param3);
        buf.extend(vec![0, 0, 0, 0, 0, 0]); // six dummy bytes

        for i in 0..SUBMIX_COUNT {
            buf.extend(self.submixes[i].to_bytes());
        }

        buf
    }
}

impl SystemExclusiveData for EffectPatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        // data bytes 4...9 are the dummy bytes,
        // submix settings start at 10 with three bytes each
        let mut submixes = [Default::default(); SUBMIX_COUNT];

        let mut offset = 10;
        let mut i = 0;
        while i < SUBMIX_COUNT {
            submixes[i] = SubmixSettings {
                pan: data[offset] as i32 - 7,
                send1: data[offset + 1] as u32,
                send2: data[offset + 2] as u32,
            };
            offset += 3;
            i += 1;
        }

        EffectPatch {
            effect: Effect::try_from(data[0] + 1).unwrap(),
            param1: data[1],
            param2: data[2],
            param3: data[3],
            submixes,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size(&self) -> usize { 35 }
}

impl Checksum for EffectPatch {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        ((total & 0x7F) as u8).try_into().unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SubmixSettings {
    // K4: -7~+7, stored in SysEx as 0~15
    // K4r: -7~+7, stored as 0~15 , or 16~21 / 1~6
    pub pan: i32,

    // Effect send 1 and 2 are used on K4 only
    pub send1: u32,
    pub send2: u32,
}

impl Default for SubmixSettings {
    fn default() -> Self {
        SubmixSettings {
            pan: 0,
            send1: 0,
            send2: 0,
        }
    }
}

impl SystemExclusiveData for SubmixSettings {
    fn from_bytes(data: Vec<u8>) -> Self {
        SubmixSettings {
            pan: data[0] as i32 - 7,
            send1: data[1] as u32,
            send2: data[2] as u32,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self.pan + 7).try_into().unwrap(), self.send1 as u8, self.send2 as u8]
    }

    fn data_size(&self) -> usize { 3 }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Submix {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Submix {
    pub fn name(&self) -> String {
        match self {
            Submix::A => "A".to_string(),
            Submix::B => "B".to_string(),
            Submix::C => "C".to_string(),
            Submix::D => "D".to_string(),
            Submix::E => "E".to_string(),
            Submix::F => "F".to_string(),
            Submix::G => "G".to_string(),
            Submix::H => "H".to_string(),
        }
    }
}

impl fmt::Display for Submix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_submix_name() {
        let submix = Submix::A;
        assert_eq!(submix.name(), "A");
    }


    #[test]
    fn test_effect_patch_from_bytes() {
        let data: [u8; 35] = include!("a401effect32.in");
        let patch = EffectPatch::from_bytes(data.to_vec());
        assert_eq!(patch.effect, Effect::Reverb1);
    }
}
