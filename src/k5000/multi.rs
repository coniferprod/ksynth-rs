//! Data model for multi patches ("combi" on K5000W).
//!

use std::fmt;
use bit::BitIndex;
use crate::{
    MIDIChannel,
    SystemExclusiveData,
    ParseError,
    Checksum,
    Ranged,
    vec_to_array,
};
use crate::k5000::{
    Volume,
    PatchName
};
use crate::k5000::control::VelocitySwitchSettings;
use crate::k5000::effect::{
    EffectSettings,
    EffectControl
};
use crate::k5000::source::{Key, Zone};

pub const SECTION_COUNT: usize = 4; // number of sections in a multi patch

pub const GEQ_BAND_COUNT: usize = 7;

pub struct GEQ {
    bands: [i8; GEQ_BAND_COUNT],
}

impl GEQ {
    pub fn new() -> Self {
        Self { bands: [0; GEQ_BAND_COUNT] }
    }

    pub fn from(values: Vec<i8>) -> Self {
        Self { bands: vec_to_array(values) }
    }
}

impl Default for GEQ {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemExclusiveData for GEQ {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut bands: [i8; 7] = [0; 7];
        for (index, b) in data.iter().enumerate() {
            let value: i32 = *b as i32;
            bands[index] = (value - 64) as i8;  // 58(-6) ~ 70(+6), so 64 is zero
        }
        Ok(GEQ { bands })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for i in 0..=GEQ_BAND_COUNT {
            result.push((i + 64) as u8);
        }

        result
    }

    fn data_size() -> usize {
        GEQ_BAND_COUNT
    }
}

/// Multi patch common settings.
pub struct Common {
    pub effects: EffectSettings,
    pub geq: GEQ,
    pub name: PatchName,
    pub volume: Volume,
    pub section_mutes: [bool; SECTION_COUNT],
    pub effect_control: EffectControl,
}

impl Default for Common {
    fn default() -> Self {
        Common {
            effects: Default::default(),
            geq: Default::default(),
            name: PatchName("NewMulti".to_string()),
            volume: Volume::new(100),
            section_mutes: [false; SECTION_COUNT],  // all sections muted by default
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
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("Multi/combi common data ({} bytes): {:?}", data.len(), data);

        let mut offset = 0;
        let mut size = 31;
        let mut start = offset;
        let mut end = offset + size;

        let effects_data = &data[start..end];
        let effects = EffectSettings::from_bytes(effects_data)?;
        offset += size;

        size = 7;
        end = start + size;
        let geq_data = &data[start..end];
        let geq = GEQ::from_bytes(geq_data).unwrap();
        offset += size;

        size = 8;
        start = offset;
        end = offset + size;
        let name_data = data[start..end].to_vec();
        let name = PatchName::from_bytes(&name_data).unwrap();
        eprintln!("Name = {}", name);
        offset += size;

        let mutes_byte = data[offset];
        let mut section_mutes: [bool; SECTION_COUNT] = [false; SECTION_COUNT];
        for i in 0..SECTION_COUNT {
            section_mutes[i] = mutes_byte.bit(i);
        }
        offset += 1;

        let volume = Volume::new(data[offset] as i32);
        eprintln!("Volume = {}", volume);
        offset += 1;

        size = 6;
        start = offset;
        end = start + size;
        let effect_control_data = &data[start..end];
        let effect_control = EffectControl::from_bytes(effect_control_data)?;
        eprintln!("Effect control = {:?}", effect_control);
        offset += size;

        Ok(Common {
            effects,
            geq,
            name,
            volume,
            section_mutes,
            effect_control
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.effects.to_bytes());
        result.extend(self.geq.to_bytes());
        result.extend(self.name.to_bytes());
        result.push(self.volume.value() as u8);

        let mut mute_byte = 0x00;
        for i in 0..SECTION_COUNT {
            if self.section_mutes[i] {
                mute_byte.set_bit(i, true);
            }
        }
        result.push(mute_byte);

        result.extend(self.effect_control.to_bytes());

        result
    }

    fn data_size() -> usize {
        EffectSettings::data_size()
        + GEQ::data_size()
        + PatchName::data_size()
        + 1 // volume
        + 1 // section mutes
        + EffectControl::data_size()
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
    pub receive_channel: MIDIChannel,
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
            receive_channel: MIDIChannel(1),
        }
    }
}

impl SystemExclusiveData for Section {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("Multi section data, {} bytes", data.len());

        let mut offset = 0;

        let bit_str = format!("{:02b}{:07b}", data[offset], data[offset + 1]);
        let single = u32::from_str_radix(&bit_str, 2).unwrap();
        offset += 2;

        let volume = data[offset] as u32;
        eprintln!("Volume = {}", volume);
        offset += 1;

        let pan = data[offset] as u32;
        eprintln!("Pan = {}", pan);
        offset += 1;

        let effect_path = data[offset] as u32;
        eprintln!("Effect path = {}", effect_path);
        offset += 1;

        let transpose = data[offset] as i32 - 64;  // stored as 40...88, scale to -24...+24
        eprintln!("Transpose = {}", transpose);
        offset += 1;

        let tune = data[offset] as i32 - 64; // stored as 1...127, scale to -63...+63
        eprintln!("Tune = {}", tune);
        offset += 1;

        let zone = Zone {
            low: Key { note: data[offset] },
            high: Key { note: data[offset + 1] }
        };
        offset += 2;

        let vs_data = vec![data[offset]];
        let vel_switch = VelocitySwitchSettings::from_bytes(&vs_data).unwrap();
        offset += 2;

        // Stored as 0...15, scale to 1...16, but on the K50000W it is zero.
        // FIXME: Do we need to deal with this?
        let receive_channel = MIDIChannel((data[offset] + 1) as i32);

        Ok(Section {
            single,
            volume,
            pan,
            effect_path,
            transpose,
            tune,
            zone,
            vel_switch,
            receive_channel,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let bit_str = format!("{:09b}", self.single);
        let msb = u8::from_str_radix(&bit_str[..2], 2).unwrap();
        let lsb = u8::from_str_radix(&bit_str[2..9], 2).unwrap();
        result.extend(vec![msb, lsb]);

        result.push(self.volume as u8);
        result.push(self.pan as u8);
        result.push(self.effect_path as u8);
        result.push((self.transpose + 64) as u8);
        result.push((self.tune + 64) as u8);

        result.extend(self.zone.to_bytes());
        result.extend(self.vel_switch.to_bytes());

        result.push(self.receive_channel.value() as u8);

        result
    }

    fn data_size() -> usize { 8 }
}

/// Multi patch with common settings and sections.
pub struct MultiPatch {
    pub checksum: u8,
    pub common: Common,
    pub sections: [Section; SECTION_COUNT],
}

impl Default for MultiPatch {
    fn default() -> Self {
        MultiPatch {
            checksum: 0x00,
            common: Default::default(),
            sections: [Default::default(), Default::default(), Default::default(), Default::default()]
        }
    }
}

impl SystemExclusiveData for MultiPatch {
    fn from_bytes(data: &[u8]) -> Result<MultiPatch, ParseError> {
        eprintln!("Multi");

        Ok(MultiPatch {
            checksum: data[0],
            common: Common::from_bytes(&data[1..55]).expect("valid common"),
            sections: [
                Section::from_bytes(&data[55..67]).expect("valid section"),
                Section::from_bytes(&data[67..79]).expect("valid section"),
                Section::from_bytes(&data[79..91]).expect("valid section"),
                Section::from_bytes(&data[91..103]).expect("valid section"),
            ]
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(0x00);  // FIXME: emit actual checksum

        result.extend(self.common.to_bytes());

        for section in &self.sections {
            result.extend(section.to_bytes());
        }

        result
    }

    fn data_size() -> usize { 77 }
}

#[cfg(test)]
mod tests {
    use super::{*};

    /*
    #[test]
    fn test_common_from_bytes() {
        let data = vec![

        ];
    }

    #[test]
    fn test_section_from_bytes() {

    }

    #[test]
    fn test_multi_patch_from_bytes() {
        let data: [u8; 1070] = include!("WizooIni.syx");
        let multi_patch = MultiPatch::from_bytes(data[9..].to_vec());  // skip sysex header but not checksum
        assert_eq!(multi_patch.common.name, "WizooIni");
    }
     */
}
