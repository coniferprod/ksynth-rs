//! Data model for multi patches ("combi" on K5000W).
//!

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::collections::BTreeMap;
use bit::BitIndex;
use crate::{SystemExclusiveData, Checksum};
use crate::k5000::control::{
    Polyphony, AmplitudeModulation, MacroController, SwitchControl,
    ControlDestination, Switch,
};
use crate::k5000::effect::{EffectSettings, EffectControl};
use crate::k5000::addkit::AdditiveKit;
use crate::k5000::source::Source;
use crate::k5000::SECTION_COUNT;

/// Multi patch common settings.
pub struct Common {
    pub effects: EffectSettings,
    pub geq: [i8; 7],
    pub name: String,
    pub is_muted: bool,
    pub effect_control: EffectControl,
}

impl Default for Common {
    fn default() -> Self {
        Common {
            effects: Default::default(),
            geq: [0, 0, 0, 0, 0, 0, 0],
            name: "NewMulti".to_string(),
            is_muted: false,
            effect_control: Default::default(),
        }
    }
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl SystemExclusiveData for Common {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("Multi/combi Common");

        // FIXME: Actually parse the multi bytes
        Default::default()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.effects.to_bytes());
        result.extend(self.geq.to_vec().iter().map(|n| (n + 64) as u8));
        result.extend(self.name.clone().into_bytes());  // note clone()
        result.push(self.volume as u8);
        result.push(if self.is_muted { 1 } else { 0 });  // FIXME: this is wrong, needs to be by section
        result.extend(self.effect_control.to_bytes());

        result
    }
}

/// Multi section.
pub struct Section {
    pub single: u32,  // inst no.
    pub volume: u32,
    pub pan: u32,
    pub effect_path: u32,
    pub transpose: i32,
    pub tune: i32,
    pub zone: Zone,
    pub vel_switch: VelocitySwitchSettings,
    pub receive_channel: u32,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.single)
    }
}

impl Default for Section {
    fn default() -> Self {
        Section {
            single: 0,
            volume: 127,
            pan: 0,
            effect_path: 0,
            transpose: 0,
            tune: 0,
            zone: Default::default(),
            vel_switch: Default::default(),
            receive_channel: 0,
        }
    }
}

impl SystemExclusiveData for Section {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("Multi section");

        // FIXME: Actually parse the multi bytes
        Default::default()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        // FIXME: emit the high and low bytes of the instrument number
        result.push(self.single as u8);
        result.push(self.single as u8);

        result.push(self.volume as u8);
        result.push(self.pan as u8);
        result.push(self.effect_path as u8);
        result.push((self.transpose + 24) as u8);
        result.push((self.tune + 63) as u8);

        result.extend(self.zone.to_bytes());
        result.extend(self.vel_switch.to_bytes());

        result.push(self.receive_channel as u8);

        result
    }
}

/// Multi patch with common settings and sections.
pub struct MultiPatch {
    pub common: Common,
    pub sections: [Section; SECTION_COUNT],
}

impl Default for MultiPatch {
    fn default() -> Self {
        MultiPatch {
            common: Default::default(),
            sections: [Default::default(), Default::default(), Default::default(), Default::default()]
        }
    }
}

impl SystemExclusiveData for MultiPatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("Multi");

        // FIXME: Actually parse the multi bytes
        Default::default()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        // FIXME: emit the high and low bytes of the instrument number
        result.push(self.single as u8);
        result.push(self.single as u8);

        result.push(self.volume as u8);
        result.push(self.pan as u8);
        result.push(self.effect_path as u8);
        result.push((self.transpose + 24) as u8);
        result.push((self.tune + 63) as u8);

        result.extend(self.zone.to_bytes());
        result.extend(self.vel_switch.to_bytes());

        result.push(self.receive_channel as u8);

        result
    }
}
