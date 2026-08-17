//! Data model for multi patches ("combi" on K5000W).
//!

use std::fmt;

use bit::BitIndex;
use rand::RngExt;

use syxpack::{
    SystemExclusiveData, 
    ParseError, 
    MidiChannel,
    Ranged,
    ranged_impl,
    Encoding,
    parse_or_default,
};

use crate::MidiNote;
use crate::k5000::control::VelocitySwitchSettings;
use crate::k5000::Volume;
use crate::k5000::effect::{
    EffectSettings, 
    EffectControl
};
use crate::k5000::source::{
    Zone,
    Key,
};

pub const SECTION_COUNT: usize = 4; // number of sections in a multi patch

/// Patch number (0...127, default 0).
/// SysEx storage: one byte, no adjustment.
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct PatchNumber(i32);
ranged_impl!(PatchNumber, 0, 127, 0);

impl Encoding for PatchNumber { }

/// Transpose (-24...24, default 0) for combi sections.
/// SysEx storage: one byte, 40(-24)~88(+24)
/// Adjustment: incoming -64, outgoing +64
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Transpose(i32);
ranged_impl!(Transpose, -24, 24, 0);

impl Encoding for Transpose {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// GEQ value: -6 ... +6, default 0.
/// SysEx storage: one byte, 58(-6)...70(+6).
#[derive (Debug, Clone, Copy, Eq, PartialEq)]
pub struct Frequency(i32);
ranged_impl!(Frequency, -6, 6, 0);

impl Encoding for Frequency {
    fn decode(b: u8) -> i32 {
        (b as i32) - 64
    }

    fn encode(&self) -> u8 {
        (self.value() + 64) as u8        
    }
}

/// Multi patch common settings.
pub struct Common {
    pub effects: EffectSettings,
    pub geq: [Frequency; 7],
    pub name: String,
    pub volume: Volume,
    pub section_mutes: [bool; SECTION_COUNT],
    pub effect_control: EffectControl,
}

impl Default for Common {
    fn default() -> Self {
        Common {
            effects: Default::default(),
            geq: [Default::default(); 7],
            name: "NewMulti".to_string(),
            volume: Default::default(),
            section_mutes: [false, false, false, false],  // all sections muted by default
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
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("Multi/combi common data ({} bytes): {:?}", data.len(), data);

        let mut offset = 0;
        let mut size = 31;
        let mut start = offset;
        let mut end = offset + size;

        let effects_data = &data[start..end];
        let effects = EffectSettings::parse(effects_data);
        offset += size;

        size = 7;
        end = start + size;
        let geq_data = &data[start..end];
        let mut frequencies: [Frequency; 7] = [Default::default(); 7];
        let mut f_i = 0;
        for b in geq_data {
            frequencies[f_i] = parse_or_default::<Frequency>(*b);
            f_i += 1
        }
        //let geq_values = geq_data.iter().map(|n| Frequency::from(*n));  // 58(-6) ~ 70(+6), so 64 is zero
        offset += size;

        size = 8;
        start = offset;
        end = offset + size;
        let name_data = data[start..end].to_vec();
        let name = String::from_utf8(name_data).unwrap();
        eprintln!("Name = {}", name);
        offset += size;

        let mutes_byte = data[offset];
        let mut section_mutes: [bool; SECTION_COUNT] = [false; SECTION_COUNT];
        for i in 0..SECTION_COUNT {
            section_mutes[i] = mutes_byte.bit(i);
        }
        offset += 1;

        let volume = parse_or_default::<Volume>(data[offset]);
        eprintln!("Volume = {}", volume);
        offset += 1;

        size = 6;
        start = offset;
        end = start + size;
        let effect_control_data = &data[start..end];
        let effect_control = EffectControl::parse(effect_control_data);
        eprintln!("Effect control = {:?}", effect_control);
        //offset += size;

        Ok(Common {
            effects: effects.unwrap(),
            geq: frequencies,
            name,
            volume,
            section_mutes,
            effect_control: effect_control.unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.effects.to_bytes());

        for i in 0..7 {
            result.push(self.geq[i].encode());
        }
        //result.extend(self.geq.to_vec().iter().map(|n| n.into()));

        result.extend(self.name.clone().into_bytes());  // note the use of clone() here
        result.push(self.volume.encode());

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

    fn data_size() -> usize { todo!("data size") }
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
    pub receive_channel: MidiChannel,
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
            receive_channel: MidiChannel::new(1),
        }
    }
}

impl SystemExclusiveData for Section {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
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
            low: parse_or_default::<MidiNote>(data[offset]),
            high: parse_or_default::<MidiNote>(data[offset + 1]), 
        };
        offset += 2;

        let vel_switch = VelocitySwitchSettings::parse(&vec![data[offset]]);
        offset += 2;

        // Stored as 0...15, scale to 1...16, but on the K50000W it is zero.
        // FIXME: Do we need to deal with this?
        let receive_channel = parse_or_default::<MidiChannel>(data[offset]);

        Ok(Section {
            single,
            volume,
            pan,
            effect_path,
            transpose,
            tune,
            zone,
            vel_switch: vel_switch.unwrap(),
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

        result.push(self.receive_channel.encode());

        result
    }

    fn data_size() -> usize {
        todo!("section data size")
    }
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
            sections: [
                Default::default(), 
                Default::default(), 
                Default::default(), 
                Default::default()
            ],
        }
    }
}

impl SystemExclusiveData for MultiPatch {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("Multi");

        Ok(MultiPatch {
            checksum: data[0],
            common: Common::parse(&data[1..55]).unwrap(),
            sections: [
                Section::parse(&data[55..67]).unwrap(),
                Section::parse(&data[67..79]).unwrap(),
                Section::parse(&data[79..91]).unwrap(),
                Section::parse(&data[91..103]).unwrap(),
            ],
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

    fn data_size() -> usize {
        todo!("multi data size")
    }
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
 */

    #[test]
    fn test_section_from_bytes() {

    }

    /*
    #[test]
    fn test_multi_patch_from_bytes() {
        let data: [u8; 1070] = include!("WizooIni.in");
        let multi_patch = MultiPatch::parse(data[9..].to_vec());  // skip sysex header but not checksum
        assert_eq!(multi_patch.common.name, "WizooIni");
    }
     */
}
