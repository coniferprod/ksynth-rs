use std::convert::TryFrom;
use std::fmt;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::k5000::control;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
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
        write!(f, "{:?}", self)
    }
}

pub struct EffectDefinition {
    pub effect: Effect,
    pub depth: u8,
    pub parameter1: u8,
    pub parameter2: u8,
    pub parameter3: u8,
    pub parameter4: u8,
}

impl fmt::Display for EffectDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "effect = {}\ndepth = {}\nparameter1 = {}\nparameter2 = {}\nparameter3 = {}\nparameter4 = {}",
            self.effect, self.depth, self.parameter1, self.parameter2, self.parameter3, self.parameter4)
    }
}

impl Default for EffectDefinition {
    fn default() -> Self {
        EffectDefinition {
            effect: Default::default(),
            depth: 0,
            parameter1: 0,
            parameter2: 0,
            parameter3: 0,
            parameter4: 0,
        }
    }
}

impl SystemExclusiveData for EffectDefinition {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("EffectDefinition, data = {:02X?}", data);
        EffectDefinition {
            effect: Effect::try_from(data[0]).unwrap(),
            depth: data[1],
            parameter1: data[2],
            parameter2: data[3],
            parameter3: data[4],
            parameter4: data[5],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.effect as u8, self.depth, self.parameter1, self.parameter2, self.parameter3, self.parameter4]
    }
}

pub struct EffectSettings {
    pub algorithm: u8,
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
            algorithm: 1,
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
            algorithm: data[0] + 1,  // adjust 0~3 to 1~4
            reverb: EffectDefinition::from_bytes(data[1..7].to_vec()),
            effect1: EffectDefinition::from_bytes(data[7..13].to_vec()),
            effect2: EffectDefinition::from_bytes(data[13..19].to_vec()),
            effect3: EffectDefinition::from_bytes(data[19..25].to_vec()),
            effect4: EffectDefinition::from_bytes(data[25..31].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.algorithm - 1); // adjust back to 0~3

        result.extend(self.reverb.to_bytes());
        result.extend(self.effect1.to_bytes());
        result.extend(self.effect2.to_bytes());
        result.extend(self.effect3.to_bytes());
        result.extend(self.effect4.to_bytes());

        result
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum EffectDestination {
    Effect1DryWet,
    Effect1Parameter,
    Effect2DryWet,
    Effect2Parameter,
    Effect3DryWet,
    Effect3Parameter,
    Effect4DryWet,
    Effect4Parameter,
}

impl Default for EffectDestination {
    fn default() -> Self { EffectDestination::Effect1DryWet }
}

pub struct ControlSource {
    pub source: control::ControlSource,
    pub destination: EffectDestination,
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
            destination: EffectDestination::try_from(data[1]).unwrap(),
            depth: data[2] as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.source as u8, self.destination as u8, (self.depth + 64) as u8]
    }
}

pub struct Control {
    pub source1: ControlSource,
    pub source2: ControlSource,
}

impl Default for Control {
    fn default() -> Self {
        Control {
            source1: Default::default(),
            source2: Default::default(),
        }
    }
}
impl SystemExclusiveData for Control {
    fn from_bytes(data: Vec<u8>) -> Self {
        Control {
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
