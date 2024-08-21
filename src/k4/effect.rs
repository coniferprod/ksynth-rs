//! Data model for effects.
//!

use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::collections::HashMap;

use lazy_static::lazy_static;
use num_enum::TryFromPrimitive;

use crate::k4::{
    Level,
    SUBMIX_COUNT,
    SmallEffectParameter,
    BigEffectParameter
};
use crate::{
    SystemExclusiveData,
    ParseError,
    Checksum
};

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

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Hash)]
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
        write!(f, "{}", EFFECT_NAMES[*self as usize])
    }
}

#[derive(Clone)]
pub struct EffectPatch {
    pub effect: Effect,
    pub param1: SmallEffectParameter,
    pub param2: SmallEffectParameter,
    pub param3: BigEffectParameter,
    pub submixes: [SubmixSettings; SUBMIX_COUNT],
}

lazy_static! {
    static ref EFFECT_PARAMETER_NAMES: HashMap<&'static Effect, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(&Effect::Reverb1, vec!["Pre.delay", "Rev.Time", "Tone"]);
        map.insert(&Effect::Reverb2, vec!["Pre.delay", "Rev.Time", "Tone"]);
        map.insert(&Effect::Reverb3, vec!["Pre.delay", "Rev.Time", "Tone"]);
        map.insert(&Effect::Reverb4, vec!["Pre.delay", "Rev.Time", "Tone"]);
        map.insert(&Effect::GateReverb, vec!["Pre.delay", "Gate Time", "Tone"]);
        map.insert(&Effect::ReverseGate, vec!["Pre.delay", "Gate Time", "Tone"]);
        map.insert(&Effect::NormalDelay, vec!["Feedback", "Tone", "Delay"]);
        map.insert(&Effect::StereoPanpotDelay, vec!["Feedback", "L/R Delay", "Delay"]);
        map.insert(&Effect::Chorus, vec!["Width", "Feedback", "Rate"]);
        map.insert(&Effect::OverdrivePlusFlanger, vec!["Drive", "Fl.Type", "1-2 Bal"]);
        map.insert(&Effect::OverdrivePlusNormalDelay, vec!["Drive", "Delay Time", "1-2 Bal"]);
        map.insert(&Effect::OverdrivePlusReverb, vec!["Drive", "Rev.Type", "1-2 Bal"]);
        map.insert(&Effect::NormalDelayPlusNormalDelay, vec!["Delay1", "Delay2", "1-2 Bal"]);
        map.insert(&Effect::NormalDelayPlusStereoPanpotDelay, vec!["Delay1", "Delay2", "1-2 Bal"]);
        map.insert(&Effect::ChorusPlusNormalDelay, vec!["Chorus", "Delay", "1-2 Bal"]);
        map.insert(&Effect::ChorusPlusStereoPanpotDelay, vec!["Chorus", "Delay", "1-2 Bal"]);
        map
    };
}

impl Default for EffectPatch {
    fn default() -> Self {
        EffectPatch {
            effect: Effect::Reverb1,
            param1: SmallEffectParameter::try_new(0).unwrap(),
            param2: SmallEffectParameter::try_new(0).unwrap(),
            param3: BigEffectParameter::try_new(0).unwrap(),
            submixes: [Default::default(); SUBMIX_COUNT],
        }
    }
}

impl fmt::Display for EffectPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {} = {}, {} = {}, {} = {}",
            EFFECT_NAMES[self.effect as usize],
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[0], self.param1.into_inner(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[1], self.param2.into_inner(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[2], self.param3.into_inner()
        )
    }
}

impl EffectPatch {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf = vec![
            self.effect as u8 - 1,
            (self.param1.into_inner() + 7) as u8,
            (self.param2.into_inner() + 7) as u8,
            self.param3.into_inner()
        ];

        buf.extend(vec![0, 0, 0, 0, 0, 0]); // six dummy bytes

        for i in 0..SUBMIX_COUNT {
            buf.extend(self.submixes[i].to_bytes());
        }

        buf
    }

    pub fn parameter_names(&self) -> Vec<String> {
        vec![
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[0].to_string(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[1].to_string(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[2].to_string(),
        ]
    }
}

impl SystemExclusiveData for EffectPatch {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        // data bytes 4...9 are the dummy bytes,
        // submix settings start at 10 with three bytes each
        let mut submixes = [Default::default(); SUBMIX_COUNT];

        let mut offset = 10;
        let mut i = 0;
        while i < SUBMIX_COUNT {
            submixes[i] = SubmixSettings {
                pan: data[offset] as i32 - 7,
                send1: Level::try_new(data[offset + 1]).unwrap(),
                send2: Level::try_new(data[offset + 2]).unwrap(),
            };
            offset += 3;
            i += 1;
        }

        Ok(EffectPatch {
            effect: Effect::try_from(data[0] + 1).unwrap(),
            param1: SmallEffectParameter::try_new((data[1] as i8) - 7).unwrap(),
            param2: SmallEffectParameter::try_new((data[2] as i8) - 7).unwrap(),
            param3: BigEffectParameter::try_new(data[3]).unwrap(),
            submixes,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size() -> usize { 35 }
}

impl Checksum for EffectPatch {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        (total & 0x7F) as u8
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SubmixSettings {
    // K4: -7~+7, stored in SysEx as 0~15
    // K4r: -7~+7, stored as 0~15 , or 16~21 / 1~6
    pub pan: i32,

    // Effect send 1 and 2 are used on K4 only
    pub send1: Level,
    pub send2: Level,
}

impl Default for SubmixSettings {
    fn default() -> Self {
        SubmixSettings {
            pan: 0,
            send1: Level::try_new(0).unwrap(),
            send2: Level::try_new(0).unwrap(),
        }
    }
}

impl SystemExclusiveData for SubmixSettings {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(SubmixSettings {
            pan: data[0] as i32 - 7,
            send1: Level::try_new(data[1]).unwrap(),
            send2: Level::try_new(data[2]).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            (self.pan + 7).try_into().unwrap(),
            self.send1.into_inner(),
            self.send2.into_inner()
        ]
    }

    fn data_size() -> usize { 3 }
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
    use crate::k4::{
        bank,
        sysex::Header,
        single::SinglePatch,
        multi::MultiPatch,
        drum::DrumPatch
    };
    
    use super::{*};

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_submix_name() {
        let submix = Submix::A;
        assert_eq!(submix.name(), "A");
    }

    #[test]
    fn test_effect_patch_from_bytes() {
        let start: usize = dbg!(
            2 +
            Header::data_size() + 
            bank::SINGLE_PATCH_COUNT * SinglePatch::data_size() +
            bank::MULTI_PATCH_COUNT * MultiPatch::data_size() +
            DrumPatch::data_size());
        
        let patch = EffectPatch::from_bytes(&DATA[start..]);
        assert_eq!(patch.unwrap().effect, Effect::Reverb1);
    }

    #[test]
    fn test_effect_parameter_names() {
        let effect = EffectPatch {
            effect: Effect::Reverb1,
            param1: SmallEffectParameter::try_new(7).unwrap(),
            param2: SmallEffectParameter::try_new(5).unwrap(),
            param3: BigEffectParameter::try_new(31).unwrap(),
            submixes: [Default::default(); SUBMIX_COUNT],
        };

        if let Some(param_names) = EFFECT_PARAMETER_NAMES.get(&effect.effect) {
            assert_eq!(param_names[0], "Pre.delay");
        }
        else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn test_effect_get_parameter_names() {
        let effect = EffectPatch {
            effect: Effect::Reverb1,
            param1: SmallEffectParameter::try_new(7).unwrap(),
            param2: SmallEffectParameter::try_new(5).unwrap(),
            param3: BigEffectParameter::try_new(31).unwrap(),
            submixes: [Default::default(); SUBMIX_COUNT],
        };

        assert_eq!(effect.parameter_names(), vec!["Pre.delay", "Rev.Time", "Tone"]);
    }
}
