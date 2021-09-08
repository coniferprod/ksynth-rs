//! Data model for effect settings.
//!

use std::convert::TryFrom;
use std::fmt;
use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use lazy_static::lazy_static;

use crate::SystemExclusiveData;
use crate::k5000::control;
use crate::k5000::{Depth, UnsignedLevel};

type EffectParameter = UnsignedLevel;

static EFFECT_NAMES: &[&str] = &[
    "None",  // just to align with 1...16
    "Hall 1",
    "Hall 2",
    "Hall 3",
    "Room 1",
    "Room 2",
    "Room 3",
    "Plate 1",
    "Plate 2",
    "Plate 3",
    "Reverse",
    "Early Reflection 1",
    "Early Reflection 2",
    "Tap Delay 1",
    "Tap Delay 2",
    "Single Delay",
    "Dual Delay",
    "Stereo Delay",
    "Cross Delay",
    "Auto Pan",
    "Auto Pan & Delay",
    "Chorus 1",
    "Chorus 2",
    "Chorus 1 & Delay",
    "Chorus 2 & Delay",
    "Flanger 1",
    "Flanger 2",
    "Flanger 1 & Delay",
    "Flanger 2 & Delay",
    "Ensemble",
    "Ensemble & Delay",
    "Celeste",
    "Celeste & Delay",
    "Tremolo",
    "Tremolo & Delay",
    "Phaser 1",
    "Phaser 2",
    "Phaser 1 & Delay",
    "Phaser 2 & Delay",
    "Rotary",
    "Auto Wah",
    "Bandpass",
    "Exciter",
    "Enhancer",
    "Overdrive",
    "Distortion",
    "Overdrive & Delay",
    "Distortion & Delay",
];

/// Effect type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Hash)]
#[repr(u8)]
pub enum Effect {
    Hall1,
    Hall2,
    Hall3,
    Room1,
    Room2,
    Room3,
    Plate1,
    Plate2,
    Plate3,
    Reverse,
    LongDelay,
    EarlyReflection1,
    EarlyReflection2,
    TapDelay1,
    TapDelay2,
    SingleDelay,
    DualDelay,
    StereoDelay,
    CrossDelay,
    AutoPan,
    AutoPanAndDelay,
    Chorus1,
    Chorus2,
    Chorus1AndDelay,
    Chorus2AndDelay,
    Flanger1,
    Flanger2,
    Flanger1AndDelay,
    Flanger2AndDelay,
    Ensemble,
    EnsembleAndDelay,
    Celeste,
    CelesteAndDelay,
    Tremolo,
    TremoloAndDelay,
    Phaser1,
    Phaser2,
    Phaser1AndDelay,
    Phaser2AndDelay,
    Rotary,
    AutoWah,
    Bandpass,
    Exciter,
    Enhancer,
    Overdrive,
    Distortion,
    OverdriveAndDelay,
    DistortionAndDelay,
}

impl Default for Effect {
    fn default() -> Self { Effect::Hall1 }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", EFFECT_NAMES[*self as usize].to_string())
    }
}

lazy_static! {
    static ref EFFECT_PARAMETER_NAMES: HashMap<&'static Effect, Vec<&'static str>> = {
        let mut map = HashMap::new();
        /*  0 */ map.insert(&Effect::Hall1, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  1 */ map.insert(&Effect::Hall2, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  2 */ map.insert(&Effect::Hall3, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  3 */ map.insert(&Effect::Room1, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  4 */ map.insert(&Effect::Room2, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  5 */ map.insert(&Effect::Room3, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  6 */ map.insert(&Effect::Plate1, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  7 */ map.insert(&Effect::Plate2, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  8 */ map.insert(&Effect::Plate3, vec!["Dry/Wet 2", "Reverb Time", "Predelay Time", "High Frequency Damping"]);
        /*  9 */ map.insert(&Effect::Reverse, vec!["Dry/Wet 2", "Feedback", "Predelay Time", "High Frequency Damping"]);
        /* 10 */ map.insert(&Effect::LongDelay, vec!["Dry/Wet 2", "Feedback", "Delay Time", "High Frequency Damping"]);
        /* 11 */ map.insert(&Effect::EarlyReflection1, vec!["Slope", "Predelay Time", "Feedback", "?"]);
        /* 12 */ map.insert(&Effect::EarlyReflection2, vec!["Slope", "Predelay Time", "Feedback", "?"]);
        /* 13 */ map.insert(&Effect::TapDelay1, vec!["Delay Time 1", "Tap Level", "Delay Time 2", "?"]);
        /* 14 */ map.insert(&Effect::TapDelay2, vec!["Delay Time 1", "Tap Level", "Delay Time 2", "?"]);
        /* 15 */ map.insert(&Effect::SingleDelay, vec!["Delay Time Fine", "Delay Time Coarse", "Feedback", "?"]);
        /* 16 */ map.insert(&Effect::DualDelay, vec!["Delay Time Left", "Feedback Left", "Delay Time Right", "Feedback Right"]);
        /* 17 */ map.insert(&Effect::StereoDelay, vec!["Delay Time", "Feedback", "?", "?"]);
        /* 18 */ map.insert(&Effect::CrossDelay, vec!["Delay Time", "Feedback", "?", "?"]);
        /* 19 */ map.insert(&Effect::AutoPan, vec!["Speed", "Depth", "Predelay Time", "Wave"]);
        /* 20 */ map.insert(&Effect::AutoPanAndDelay, vec!["Speed", "Depth", "Delay Time", "Wave"]);
        /* 21 */ map.insert(&Effect::Chorus1, vec!["Speed", "Depth", "Predelay Time", "Wave"]);
        /* 22 */ map.insert(&Effect::Chorus2, vec!["Speed", "Depth", "Predelay Time", "Wave"]);
        /* 23 */ map.insert(&Effect::Chorus1AndDelay, vec!["Speed", "Depth", "Delay Time", "Wave"]);
        /* 24 */ map.insert(&Effect::Chorus2AndDelay, vec!["Speed", "Depth", "Delay Time", "Wave"]);
        /* 25 */ map.insert(&Effect::Flanger1, vec!["Speed", "Depth", "Predelay Time", "Feedback"]);
        /* 26 */ map.insert(&Effect::Flanger2, vec!["Speed", "Depth", "Predelay Time", "Feedback"]);
        /* 27 */ map.insert(&Effect::Flanger1AndDelay, vec!["Speed", "Depth", "Delay Time", "Feedback"]);
        /* 28 */ map.insert(&Effect::Flanger2AndDelay, vec!["Speed", "Depth", "Delay Time", "Feedback"]);
        /* 29 */ map.insert(&Effect::Ensemble, vec!["Depth", "Predelay Time", "?", "?"]);
        /* 30 */ map.insert(&Effect::EnsembleAndDelay, vec!["Depth", "Delay Time", "?", "?"]);
        /* 31 */ map.insert(&Effect::Celeste, vec!["Speed", "Depth", "Predelay Time", "?"]);
        /* 32 */ map.insert(&Effect::CelesteAndDelay, vec!["Speed", "Depth", "Delay Time", "?"]);
        /* 33 */ map.insert(&Effect::Tremolo, vec!["Speed", "Depth", "Predelay Time", "Wave"]);
        /* 34 */ map.insert(&Effect::TremoloAndDelay, vec!["Speed", "Depth", "Delay Time", "Wave"]);
        /* 35 */ map.insert(&Effect::Phaser1, vec!["Speed", "Depth", "Predelay Time", "Feedback"]);
        /* 36 */ map.insert(&Effect::Phaser2, vec!["Speed", "Depth", "Predelay Time", "Feedback"]);
        /* 37 */ map.insert(&Effect::Phaser1AndDelay, vec!["Speed", "Depth", "Delay Time", "Feedback"]);
        /* 38 */ map.insert(&Effect::Phaser2AndDelay, vec!["Speed", "Depth", "Delay Time", "Feedback"]);
        /* 39 */ map.insert(&Effect::Rotary, vec!["Slow Speed", "Fast Speed", "Acceleration", "Slow/Fast Switch"]);
        /* 40 */ map.insert(&Effect::AutoWah, vec!["Sense", "Frequency Bottom", "Frequency Top", "Resonance"]);
        /* 41 */ map.insert(&Effect::Bandpass, vec!["Center Frequency", "Bandwidth", "?", "?"]);
        /* 42 */ map.insert(&Effect::Exciter, vec!["EQ Low", "EQ High", "Intensity", "?"]);
        /* 43 */ map.insert(&Effect::Enhancer, vec!["EQ Low", "EQ High", "Intensity", "?"]);
        /* 44 */ map.insert(&Effect::Overdrive, vec!["EQ Low", "EQ High", "Output Level", "Drive"]);
        /* 45 */ map.insert(&Effect::Distortion, vec!["EQ Low", "EQ High", "Output Level", "Drive"]);
        /* 46 */ map.insert(&Effect::OverdriveAndDelay, vec!["EQ Low", "EQ High", "Delay Time", "Drive"]);
        /* 47 */ map.insert(&Effect::DistortionAndDelay, vec!["EQ Low", "EQ High", "Delay Time", "Drive"]);
        map
    };
}

/// Effect algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Hash)]
#[repr(u8)]
pub enum EffectAlgorithm {
    Algorithm1,
    Algorithm2,
    Algorithm3,
    Algorithm4,
}

impl fmt::Display for EffectAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EffectAlgorithm::Algorithm1 => "Algorithm 1",
            EffectAlgorithm::Algorithm2 => "Algorithm 2",
            EffectAlgorithm::Algorithm3 => "Algorithm 3",
            EffectAlgorithm::Algorithm4 => "Algorithm 4",
        })
    }
}

/// Effect definition.
pub struct EffectDefinition {
    pub effect: Effect,
    pub depth: Depth,
    pub parameter1: EffectParameter,
    pub parameter2: EffectParameter,
    pub parameter3: EffectParameter,
    pub parameter4: EffectParameter,
}

impl fmt::Display for EffectDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, depth = {}, {} = {}, {} = {}, {} = {}, {} = {}",
            EFFECT_NAMES[self.effect as usize].to_string(),
            self.depth.as_byte(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[0], self.parameter1.as_byte(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[1], self.parameter2.as_byte(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[2], self.parameter3.as_byte(),
            EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[3], self.parameter4.as_byte()
        )
    }
}

impl Default for EffectDefinition {
    fn default() -> Self {
        EffectDefinition {
            effect: Default::default(),
            depth: Depth::from(0),
            parameter1: EffectParameter::from(0),
            parameter2: EffectParameter::from(0),
            parameter3: EffectParameter::from(0),
            parameter4: EffectParameter::from(0),
        }
    }
}

impl SystemExclusiveData for EffectDefinition {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("EffectDefinition, data = {:02X?}", data);
        EffectDefinition {
            effect: Effect::try_from(data[0]).unwrap(),
            depth: Depth::from(data[1]),
            parameter1: EffectParameter::from(data[2]),
            parameter2: EffectParameter::from(data[3]),
            parameter3: EffectParameter::from(data[4]),
            parameter4: EffectParameter::from(data[5]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.effect as u8,
            self.depth.as_byte(),
            self.parameter1.as_byte(),
            self.parameter2.as_byte(),
            self.parameter3.as_byte(),
            self.parameter4.as_byte()
        ]
    }
}

/// Effect settings.
pub struct EffectSettings {
    pub algorithm: EffectAlgorithm,
    pub reverb: EffectDefinition,
    pub effect1: EffectDefinition,
    pub effect2: EffectDefinition,
    pub effect3: EffectDefinition,
    pub effect4: EffectDefinition,
}

impl fmt::Display for EffectSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "alg = {}\nreverb = {}\neffect1 = {}\neffect2 = {}\neffect3 = {}\neffect4 = {}",
            self.algorithm, self.reverb, self.effect1, self.effect2, self.effect3, self.effect4)
    }
}

impl Default for EffectSettings {
    fn default() -> Self {
        EffectSettings {
            algorithm: EffectAlgorithm::Algorithm1,
            reverb: Default::default(),
            effect1: Default::default(),
            effect2: Default::default(),
            effect3: Default::default(),
            effect4: Default::default(),
        }
    }
}

impl SystemExclusiveData for EffectSettings {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("EffectSettings");
        EffectSettings {
            algorithm: EffectAlgorithm::try_from(data[0]).unwrap(),  // 0~3 to enum
            reverb: EffectDefinition::from_bytes(data[1..7].to_vec()),
            effect1: EffectDefinition::from_bytes(data[7..13].to_vec()),
            effect2: EffectDefinition::from_bytes(data[13..19].to_vec()),
            effect3: EffectDefinition::from_bytes(data[19..25].to_vec()),
            effect4: EffectDefinition::from_bytes(data[25..31].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.algorithm as u8); // enum raw value maps to 0~3

        result.extend(self.reverb.to_bytes());
        result.extend(self.effect1.to_bytes());
        result.extend(self.effect2.to_bytes());
        result.extend(self.effect3.to_bytes());
        result.extend(self.effect4.to_bytes());

        result
    }
}

/// Effect destinations.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Destination {
    Effect1DryWet,
    Effect1Parameter,
    Effect2DryWet,
    Effect2Parameter,
    Effect3DryWet,
    Effect3Parameter,
    Effect4DryWet,
    Effect4Parameter,
}

impl Default for Destination {
    fn default() -> Self { Destination::Effect1DryWet }
}

/// Effect control source.
pub struct ControlSource {
    pub source: control::ControlSource,
    pub destination: Destination,
    pub depth: i8,
}

impl Default for ControlSource {
    fn default() -> Self {
        ControlSource {
            source: Default::default(),
            destination: Default::default(),
            depth: 0,
        }
    }
}

impl SystemExclusiveData for ControlSource {
    fn from_bytes(data: Vec<u8>) -> Self {
        ControlSource {
            source: control::ControlSource::try_from(data[0]).unwrap(),
            destination: Destination::try_from(data[1]).unwrap(),
            depth: data[2] as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.source as u8, self.destination as u8, (self.depth + 64) as u8]
    }
}

/// Effect control with two sources.
pub struct EffectControl {
    pub source1: ControlSource,
    pub source2: ControlSource,
}

impl Default for EffectControl {
    fn default() -> Self {
        EffectControl {
            source1: Default::default(),
            source2: Default::default(),
        }
    }
}
impl SystemExclusiveData for EffectControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        EffectControl {
            source1: ControlSource::from_bytes(data[0..3].to_vec()),
            source2: ControlSource::from_bytes(data[3..6].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.source1.to_bytes());
        result.extend(self.source2.to_bytes());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_effect_parameter_names() {
        let effect = EffectDefinition {
            effect: Effect::Hall1,
            depth: Depth::from(100),
            parameter1: EffectParameter::from(7),
            parameter2: EffectParameter::from(5),
            parameter3: EffectParameter::from(31),
            parameter4: EffectParameter::from(0),
        };

        if let Some(param_names) = EFFECT_PARAMETER_NAMES.get(&effect.effect) {
            assert_eq!(param_names[1], "Reverb Time");
        }
        else {
            assert_eq!(true, false);
        }
    }
}
