//! Data model for effect settings.
//!

use std::convert::TryFrom;
use std::fmt;
use std::collections::HashMap;

use num_enum::TryFromPrimitive;
use lazy_static::lazy_static;
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

use crate::{
    SystemExclusiveData,
    ParseError,
    XMLData,
    make_xml_element,
};
use crate::k5000::{ByteValue, DESCRIPTORS, control};

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
#[derive(
    Debug, Copy, Clone, 
    Eq, PartialEq, Hash,
    Default, TryFromPrimitive,
    Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Effect {
    #[default]
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

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", EFFECT_NAMES[*self as usize])
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
#[derive(
    Debug, Copy, Clone, 
    Eq, PartialEq, Hash,
    Default, TryFromPrimitive,
    Serialize, Deserialize,
)]
#[repr(u8)]
pub enum EffectAlgorithm {
    #[default]
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

const PARAMETER_COUNT: usize = 4;

/// Effect definition.
#[derive(
    Debug, Clone, Copy, 
    PartialEq, Eq,
    Serialize, Deserialize
)]
pub struct EffectDefinition {
    pub effect: Effect,  // reverb = 0~10, others = 11~47
    pub depth: i32, // Depth,  // 0~100

    pub parameters: [i32; PARAMETER_COUNT],  // EffectParameter,  // 0~127
}

impl fmt::Display for EffectDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, 
            "{}, depth = {}",
            EFFECT_NAMES[self.effect as usize],
            self.depth);

        for i in 0..PARAMETER_COUNT {
            let _ = write!(f, 
                "{} = {}", 
                EFFECT_PARAMETER_NAMES.get(&self.effect).unwrap()[i],
                self.parameters[i]);
        }
        writeln!(f, "")
    }
}

impl Default for EffectDefinition {
    fn default() -> Self {
        Self {
            effect: Default::default(),
            depth: Default::default(),
            parameters: [Default::default(); PARAMETER_COUNT],
        }
    }
}

impl SystemExclusiveData for EffectDefinition {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("EffectDefinition, data = {:02X?}", data);
        let depth_desc = DESCRIPTORS.get(&ByteValue::Depth).unwrap();
        let param_desc = DESCRIPTORS.get(&ByteValue::EffectParameter).unwrap();

        Ok(Self {
            effect: Effect::try_from(data[0]).unwrap(),  // 11~47
            depth: (depth_desc.incoming)(data[1]),
            parameters: [
                (param_desc.incoming)(data[2]),
                (param_desc.incoming)(data[3]),
                (param_desc.incoming)(data[4]),
                (param_desc.incoming)(data[5]),
            ],
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let depth_desc = DESCRIPTORS.get(&ByteValue::Depth).unwrap();
        let param_desc = DESCRIPTORS.get(&ByteValue::EffectParameter).unwrap();

        vec![
            self.effect as u8,
            (depth_desc.outgoing)(self.depth),
            (param_desc.outgoing)(self.parameters[0]),
            (param_desc.outgoing)(self.parameters[1]),
            (param_desc.outgoing)(self.parameters[2]),
            (param_desc.outgoing)(self.parameters[3]),
        ]
    }

    fn data_size() -> usize { 6 }
}

const EFFECT_COUNT: usize = 4;

/// Effect settings.
#[derive(
    Debug, Copy, Clone, 
    PartialEq, Eq,
    Serialize, Deserialize,
)]
pub struct EffectSettings {
    pub algorithm: EffectAlgorithm,  // 0~3
    pub reverb: EffectDefinition,
    pub effects: [EffectDefinition; EFFECT_COUNT],
}

impl fmt::Display for EffectSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Algorithm: {}", self.algorithm);
        let _ = writeln!(f, "Reverb: {}", self.reverb);
        for i in 0..EFFECT_COUNT {
            let _ = writeln!(f, "Effect{}: {}", i + 1, self.effects[i]);
        }
        write!(f, "")
    }
}

impl Default for EffectSettings {
    fn default() -> Self {
        Self {
            algorithm: EffectAlgorithm::Algorithm1,
            reverb: Default::default(),
            effects: [Default::default(); EFFECT_COUNT],
        }
    }
}

impl SystemExclusiveData for EffectSettings {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("EffectSettings, data = {:02X?}", data);
        let effects = [
            EffectDefinition::from_bytes(&data[7..13])?,
            EffectDefinition::from_bytes(&data[13..19])?,
            EffectDefinition::from_bytes(&data[19..25])?,
            EffectDefinition::from_bytes(&data[25..31])?,
        ];
        Ok(Self {
            algorithm: EffectAlgorithm::try_from(data[0]).unwrap(),  // 0~3 to enum
            reverb: EffectDefinition::from_bytes(&data[1..7])?,
            effects, 
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.algorithm as u8); // enum raw value maps to 0~3

        result.extend(self.reverb.to_bytes());
        for i in 0..EFFECT_COUNT {
            result.extend(self.effects[i].to_bytes());
        }

        result
    }

    fn data_size() -> usize { 31 }
}

/*
impl XMLData for EffectSettings {
    fn to_xml(&self) -> XMLElement {
        self.to_xml_named("effect-settings")
    }

    fn to_xml_named(&self, name: &str) -> XMLElement {
        let mut e = XMLElement::new(name);

        let alg_e = XMLElement::new("algoritnm");
        e.add_text(match self.algorithm {
            EffectAlgorithm::Algorithm1 => "1".to_string(),
            EffectAlgorithm::Algorithm2 => "2".to_string(),
            EffectAlgorithm::Algorithm3 => "3".to_string(),
            EffectAlgorithm::Algorithm4 => "4".to_string(),
        });
        e.add_child(alg_e);

        let reverb_e = XMLElement::new("reverb");

        e.add_child(reverb_e);
        e
    }
}
 */

/// Effect destinations.
#[derive(
    Debug, Copy, Clone, 
    Eq, PartialEq, 
    Default, TryFromPrimitive,
    Serialize, Deserialize,
)]
#[repr(u8)]
pub enum EffectDestination {
    #[default]
    Effect1DryWet,

    Effect1Parameter,
    Effect2DryWet,
    Effect2Parameter,
    Effect3DryWet,
    Effect3Parameter,
    Effect4DryWet,
    Effect4Parameter,
}

/// Effect control source.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ControlSource {
    pub source: control::ControlSource,  // 0~13
    pub destination: EffectDestination,  // 0~9
    pub depth: i32, // Depth, // (-31)33~(+31)95
}

impl SystemExclusiveData for ControlSource {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let depth_desc = DESCRIPTORS.get(&ByteValue::Depth).unwrap();

        Ok(Self {
            source: control::ControlSource::try_from(data[0]).unwrap(),
            destination: EffectDestination::try_from(data[1]).unwrap(),
            depth: (depth_desc.incoming)(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let depth_desc = DESCRIPTORS.get(&ByteValue::Depth).unwrap();

        vec![
            self.source as u8, 
            self.destination as u8, 
            (depth_desc.outgoing)(self.depth),
        ]
    }

    fn data_size() -> usize { 3 }
}

/// Effect control with two sources.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectControl {
    pub source1: ControlSource,
    pub source2: ControlSource,
}

impl SystemExclusiveData for EffectControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {
            source1: ControlSource::from_bytes(&data[0..3])?,
            source2: ControlSource::from_bytes(&data[3..6])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend(self.source1.to_bytes());
        result.extend(self.source2.to_bytes());
        result
    }

    fn data_size() -> usize { 6 }
}

#[cfg(test)]
mod tests {
    use super::{*};

    use serde::{Deserialize, Serialize};
    use serde_xml_rs::{from_str, to_string};

    #[test]
    fn test_serialize_effect_settings() {
        let e: EffectSettings = Default::default();
        println!("{}", e);
        match to_string(&e) {
            Ok(s) => println!("EffectSettings as XML = {}", s),
            Err(e) => eprintln!("error serializing EffectSettings: {}", e),
        }
    }

    #[test]
    fn test_serialize_effect() {
        let e: Effect = Default::default();
        match to_string(&e) {
            Ok(s) => println!("Effect as XML = {}", s),
            Err(e) => eprintln!("error serializing Effect: {}", e),
        }
    }

    #[test]
    fn test_serialize_effect_definition() {
        let e: EffectDefinition = Default::default();
        match to_string(&e) {
            Ok(s) => println!("EffectDefinition as XML = {}", s),
            Err(e) => eprintln!("error serializing EffectDefinition: {}", e),
        }
    }

    #[test]
    fn test_serialize_effect_algorithm() {
        let e: EffectAlgorithm = Default::default();
        match to_string(&e) {
            Ok(s) => println!("EffectAlgorithm as XML = {}", s),
            Err(e) => eprintln!("error serializing EffectAlgorithm: {}", e),
        }
    }

    #[test]
    fn test_effect_parameter_names() {
        let effect = EffectDefinition {
            effect: Effect::Hall1,
            depth: 100,
            parameters: [7, 5, 31, 0],
        };

        if let Some(param_names) = EFFECT_PARAMETER_NAMES.get(&effect.effect) {
            assert_eq!(param_names[1], "Reverb Time");
        }
        else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn test_effect_settings_from_bytes() {
        let data = vec![
            // Effect data
            0x00,  // effect algorithm
            0x00,  // reverb type
            0x02,  // reverb dry/wet
            0x02,  // reverb param 1
            0x0d,  // reverb param 2
            0x41,  // reverb param 3
            0x0a,  // reverb param 4
            0x10,  // effect 1 type
            0x00,  // effect 1 depth
            0x58,  // effect 1 param 1
            0x33,  // effect 1 param 2
            0x69,  // effect 1 param 3
            0x22,  // effect 1 param 4
            0x1d, 0x00, 0x4a, 0x00, 0x00, 0x00,  // effect 2 (as above)
            0x24, 0x00, 0x04, 0x3a, 0x04, 0x38,  // effect 3 (as above)
            0x2a, 0x00, 0x0c, 0x0c, 0x63, 0x00,  // effect 4 (as above)
        ];

        let effect_settings = EffectSettings::from_bytes(&data);
        assert_eq!(effect_settings.unwrap().effects[3].parameters[2], 0x63);
    }
}
