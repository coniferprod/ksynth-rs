//! Data model for single patches.
//!

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::collections::BTreeMap;

use bit::BitIndex;

use crate::{SystemExclusiveData, ParseError, Checksum};
use crate::k5000::control::{
    Polyphony, AmplitudeModulation, MacroController, SwitchControl,
    ControlDestination, Switch
};
use crate::k5000::effect::{EffectSettings, EffectControl};
use crate::k5000::addkit::AdditiveKit;
use crate::k5000::source::Source;
use crate::k5000::{Volume, MacroParameterDepth, PortamentoLevel};

/// Portamento setting.
pub enum Portamento {
    Off,
    On(PortamentoLevel)
}

impl fmt::Display for Portamento {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Portamento::Off => write!(f, "{}", "OFF"),
            Portamento::On(speed) => write!(f, "{}", speed),
        }
    }
}

/// Single patch common data.
pub struct Common {
    pub effects: EffectSettings,
    pub name: String,
    pub volume: Volume,
    pub polyphony: Polyphony,
    pub source_count: u8,
    pub source_mutes: [bool; 6],
    pub amplitude_modulation: AmplitudeModulation,
    pub effect_control: EffectControl,
    pub portamento: Portamento,
    pub macros: [MacroController; 4],
    pub switches: SwitchControl,
    pub geq: [i8; 7],
}

impl Default for Common {
    fn default() -> Self {
        Common {
            effects: Default::default(),
            name: "NewSound".to_string(),
            volume: Volume::new(99),
            polyphony: Polyphony::Poly,
            source_count: 2,
            source_mutes: [false, false, true, true, true, true],
            amplitude_modulation: Default::default(),
            effect_control: Default::default(),
            portamento: Portamento::Off,
            macros: [Default::default(), Default::default(), Default::default(), Default::default()],
            switches: Default::default(),
            geq: [0, 0, 0, 0, 0, 0, 0],
        }
    }
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{}\nVolume: {:3}  Sources: {}  Poly: {}  AM: {}  Portamento: {}\n\nMacro Controllers:\nUser 1: {}\nUser 2: {}\nUser 3: {}\nUser 4: {}\n\nSwitches:\nSwitch 1: {}   FootSw1: {}\nSwitch 2: {}   FootSw2: {}\n\nEffect settings:\n{}\n",
            self.name, self.volume, self.source_count, self.polyphony, self.amplitude_modulation,
            self.portamento, self.macros[0], self.macros[1], self.macros[2], self.macros[3],
            self.switches.switch1, self.switches.footswitch1, self.switches.switch2, self.switches.footswitch2,
            self.effects
        )
    }
}

fn vec_to_array(v: Vec<i8>) -> [i8; 7] {
    v.try_into()
        .unwrap_or_else(|v: Vec<i8>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()))
}

impl SystemExclusiveData for Common {
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        eprintln!("Common data ({} bytes): {:?}", data.len(), data);

        let mut offset = 0;
        let mut size = 31;
        let mut start = offset;
        let mut end = offset + size;
        let effects_data = data[start..end].to_vec();
        let effects = EffectSettings::from_bytes(effects_data);
        offset += size;

        eprintln!("GEQ data at offset {}", offset + 1);
        size = 7;
        end = start + size;
        let geq_data = data[start..end].to_vec();

        let geq_values = geq_data.iter().map(|n| *n as i8 - 64).collect();  // 58(-6) ~ 70(+6), so 64 is zero
        offset += size;

        eprintln!("Drum mark at offset {}", offset + 1);
        offset += 1;  // skip the drum mark
        eprintln!("Skipped 'drum mark'");

        size = 8;
        start = offset;
        end = offset + size;
        let name_data = data[start..end].to_vec();
        let name = String::from_utf8(name_data).unwrap();
        eprintln!("Name at offset {}", offset + 1);
        eprintln!("Name = {}", name);
        offset += size;

        let volume = Volume::from(data[offset]);
        eprintln!("Volume = {}", volume);
        offset += 1;

        let polyphony = Polyphony::try_from(data[offset]).unwrap();
        eprintln!("Polyphony = {}", polyphony);
        offset += 1;

        offset += 1;  // skip the "no use" byte
        eprintln!("Skipped 'no use'");

        let source_count = data[offset];
        eprintln!("Sources: {}", source_count);
        offset += 1;

        let mutes_byte = data[offset];
        let mut source_mutes: [bool; 6] = [false; 6];
        for i in 0..6 {
            source_mutes[i] = mutes_byte.bit(i);
        }
        offset += 1;

        let amplitude_modulation = AmplitudeModulation::try_from(data[offset]).unwrap();
        eprintln!("AM = {}", amplitude_modulation);
        offset += 1;

        size = 6;
        start = offset;
        end = start + size;
        let effect_control_data = data[start..end].to_vec();
        let effect_control = EffectControl::from_bytes(effect_control_data);
        eprintln!("Effect control = {:?}", effect_control);
        offset += size;

        let portamento = if data[offset] == 1 {
            Portamento::On(PortamentoLevel::from(data[offset + 1]))
        } else {
            Portamento::Off
        };
        eprintln!("Portamento: {}", portamento);
        offset += 2;

        let mut macro_destinations = Vec::<u8>::new();
        for i in 0..8 {
            eprintln!("Macro {} destination: data = {}", i + 1, data[offset]);
            macro_destinations.push(data[offset]);
            offset += 1;
        }
        let mut macro_depths = Vec::<u8>::new();
        for i in 0..8 {
            eprintln!("Macro {} depth: data = {}", i + 1, data[offset]);
            macro_depths.push(data[offset]);
            offset += 1;
        }

        let macros: [MacroController; 4] = [
            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[0]).unwrap(),
                depth1: MacroParameterDepth::from(macro_depths[0]),
                destination2: ControlDestination::try_from(macro_destinations[1]).unwrap(),
                depth2: MacroParameterDepth::from(macro_depths[1]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[2]).unwrap(),
                depth1: MacroParameterDepth::from(macro_depths[2]),
                destination2: ControlDestination::try_from(macro_destinations[3]).unwrap(),
                depth2: MacroParameterDepth::from(macro_depths[3]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[4]).unwrap(),
                depth1: MacroParameterDepth::from(macro_depths[4]),
                destination2: ControlDestination::try_from(macro_destinations[5]).unwrap(),
                depth2: MacroParameterDepth::from(macro_depths[5]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[6]).unwrap(),
                depth1: MacroParameterDepth::from(macro_depths[6]),
                destination2: ControlDestination::try_from(macro_destinations[7]).unwrap(),
                depth2: MacroParameterDepth::from(macro_depths[7]),
            },
        ];

        let switches = SwitchControl {
            switch1: Switch::try_from(data[offset]).unwrap(),
            switch2: Switch::try_from(data[offset + 1]).unwrap(),
            footswitch1: Switch::try_from(data[offset + 2]).unwrap(),
            footswitch2: Switch::try_from(data[offset + 3]).unwrap(),
        };
        eprintln!("Switches: {:?}", switches);

        Ok(Common {
            effects: effects?,
            geq: vec_to_array(geq_values),
            name: name,
            volume,
            polyphony,
            source_count,
            source_mutes,
            amplitude_modulation,
            effect_control: effect_control?,
            portamento,
            macros: macros,
            switches,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.effects.to_bytes());
        result.extend(self.geq.to_vec().iter().map(|n| (n + 64) as u8));
        result.push(0);  // drum_mark
        result.extend(self.name.clone().into_bytes());  // note clone()
        result.push(self.volume.into());  // converts value to u8 on the fly
        result.push(self.polyphony as u8);
        result.push(0);  // "no use"
        result.push(self.source_count);

        let mut mute_byte = 0x00;
        for i in 0..6 {
            if self.source_mutes[i] {
                mute_byte.set_bit(i, true);
            }
        }
        result.push(mute_byte);

        result.push(self.amplitude_modulation as u8);
        result.extend(self.effect_control.to_bytes());

        match self.portamento {
            Portamento::Off => {
                result.push(0);
                result.push(0);
            },
            Portamento::On(speed) => {
                result.push(1);
                result.push(speed.into());
            }
        }

        // Pick out the destinations and depths as the SysEx spec wants them.
        for m in &self.macros {
            result.push(m.destination1 as u8);
            result.push(m.destination2 as u8);
        }

        for m in &self.macros {
            result.push(m.depth1.into()); // -31(33)~+31(95)
            result.push(m.depth2.into());
        }

        result.extend(self.switches.to_bytes());

        result
    }
}

/// Single patch.
pub struct SinglePatch {
    pub common: Common,
    pub sources: Vec<Source>,
    pub additive_kits: BTreeMap<String, AdditiveKit>,  // keeps the keys in order
}

impl SinglePatch {
    /// Returns a single patch with he given number of default PCM and ADD sources.
    ///
    /// # Arguments
    /// * `pcm_count` - The number of PCM sources
    /// * `add _count` - The number of ADD sources
    pub fn new(pcm_count: u32, additive_count: u32) -> SinglePatch {
        let mut all_sources = Vec::<Source>::new();
        let mut pcm_count = pcm_count;
        while pcm_count > 0 {
            all_sources.push(Source::pcm());
            pcm_count -= 1;
        }

        let mut kits = BTreeMap::<String, AdditiveKit>::new();

        let mut add_index = 1;
        let mut additive_count = additive_count;
        while additive_count > 0 {
            all_sources.push(Source::additive());
            let key = format!("s{}", add_index);
            add_index += 1;
            kits.insert(key, AdditiveKit::new());
            additive_count -= 1;
        }

        SinglePatch {
            common: Default::default(),
            sources: all_sources,
            additive_kits: kits,
        }
    }

    pub fn get_size(data: Vec<u8>) -> usize {
        let mut offset = 0;

        let original_checksum = data[0];
        eprintln!("original checksum = {:#02x}", original_checksum);

        let common = Common::from_bytes(data[1..82].to_vec());
        offset += 81;
        let mut sources = Vec::<Source>::new();
        for _i in 0..common.unwrap().source_count {
            let source = Source::from_bytes(data[offset..offset + 86].to_vec());
            sources.push(source.unwrap());
            offset += 86;
        }

        let pcm_source_count = sources.iter().filter(|s| s.is_pcm()).count();
        let additive_source_count = sources.iter().filter(|s| s.is_additive()).count();

        (82 + pcm_source_count * 86 + additive_source_count * 462).try_into().unwrap()
    }
}

impl Checksum for SinglePatch {
    fn checksum(&self) -> u8 {
        // Bank A,D,E,F: check sum = {(common sum) + (source1 sum) [+ (source2~6 sum)] + 0xa5} & 0x7f

        let common_data = self.common.to_bytes();
        let mut common_sum: u32 = 0;
        for d in common_data.iter() {
            common_sum += (d & 0xff) as u32;
        }

        let mut total = common_sum & 0xff;

        for source in self.sources.iter() {
            let mut source_sum = 0;
            let source_data = source.to_bytes();
            for d in source_data.iter() {
                source_sum = d & 0xff;
            }

            total += (source_sum & 0xff) as u32;
        }

        total += 0xa5;

        (total & 0x7f) as u8
    }
}

impl Default for SinglePatch {
    fn default() -> Self {
        SinglePatch {
            common: Default::default(),
            sources: vec![Default::default(), Default::default()],
            additive_kits: BTreeMap::<String, AdditiveKit>::new(),
        }
    }
}

impl SystemExclusiveData for SinglePatch {
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        let mut offset: usize = 0;
        let mut start: usize;
        let mut end: usize;
        let mut size: usize;

        /*
        let original_checksum = data[offset];
        eprintln!("original checksum = {:#02x}", original_checksum);
        offset += 1;
        */

        size = 81;
        start = offset;
        end = start + size;
        let common_data = data[start..end].to_vec();
        eprintln!("Starting to parse source common...");
        let common = Common::from_bytes(common_data);
        offset += size;

        eprintln!("Starting to parse {} sources, offset = {}", common.as_ref().unwrap().source_count, offset);

        size = 86;
        let mut sources = Vec::<Source>::new();
        for i in 0..common.as_ref().unwrap().source_count {
            start = offset;
            end = start + size;
            let source_data = data[start..end].to_vec();
            eprintln!("Parsing source {}...", i + 1);
            let source = Source::from_bytes(source_data);
            sources.push(source?);
            offset += size;
        }

        let mut additive_kits = BTreeMap::<String, AdditiveKit>::new();

        // How many additive kits should we expect then?
        let kit_count = sources.iter().filter(|s| s.oscillator.wave.is_additive()).count();
        let mut kit_index = 0;
        size = 806;
        while kit_index < kit_count {
            start = offset;
            end = start + size;
            let kit_data = data[start..end].to_vec();
            let kit = AdditiveKit::from_bytes(kit_data);
            offset += size;
            let kit_name = format!("s{}", kit_index + 1);
            additive_kits.insert(kit_name, kit?);
            kit_index += 1;
        }

        Ok(SinglePatch {
            common: common?,
            sources: sources,
            additive_kits: additive_kits,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let mut total = 0;

        result.push(self.checksum());
        eprintln!("checksum, 1 byte");
        total += 1;

        let common_bytes = self.common.to_bytes();
        result.extend(&common_bytes);
        eprintln!("single common, {} bytes", common_bytes.len());
        total += common_bytes.len();

        for source in self.sources.iter() {
            let source_bytes = source.to_bytes();
            result.extend(&source_bytes);
            eprintln!("source, {} bytes", source_bytes.len());
            total += source_bytes.len();
        }

        for k in self.additive_kits.keys() {
            let kit = self.additive_kits.get(k).unwrap();
            let kit_bytes = kit.to_bytes();
            result.extend(&kit_bytes);
            eprintln!("additive kit, {} bytes", kit_bytes.len());
            total += kit_bytes.len();
        }

        eprintln!("total {} bytes", total);

        result
    }
}

impl fmt::Display for SinglePatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sources_str = String::new();
        let mut source_num = 1;
        for source in self.sources.iter() {
            sources_str.push_str(format!("Source {}:\n{}\n\n", source_num, source).as_str());
            source_num += 1;
        }
        write!(f, "{}\nSources:\n{}", self.common, sources_str)
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_common_from_bytes() {
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
            0x42, 0x41, 0x40, 0x40, 0x3f, 0x3e, 0x41, // GEQ
            0x00,  // "drum mark"
            0x57, 0x69, 0x7a, 0x6f, 0x6f, 0x49, 0x6e, 0x69,  // name "WizooIni"
            0x73,  // volume
            0x00,  // polyphony
            0x00,  // "no use"
            0x02,  // no. of sources
            0x01,  // source mutes
            0x00,  // AM

            // Effect control
            0x02, 0x01, 0x40,  // control source 1, destination, depth
            0x01, 0x03, 0x40,  // control source 2, destination, depth
            0x00, 0x00,  // portamento flag and p. speed

            // Macro controllers
            0x00, 0x00,  // macro controller 1 params 1 and 2
            0x00, 0x00,  // macro controller 2 params 1 and 2
            0x00, 0x00,  // macro controller 3 params 1 and 2
            0x00, 0x00,  // macro controller 4 params 1 and 2
            0x40, 0x40,  // macro controller 1 param 1 and param 2 depths
            0x40, 0x40,  // macro controller 2 param 1 and param 2 depths
            0x40, 0x40,  // macro controller 3 param 1 and param 2 depths
            0x40, 0x40,  // macro controller 4 param 1 and param 2 depths
            0x00, 0x00, 0x00, 0x00,  // SW1, SW2, F.SW1, F.SW2
        ];

        let common = Common::from_bytes(data);
        assert_eq!(common.unwrap().name, "WizooIni");
    }

    #[test]
    fn test_single_patch_from_bytes() {
        let data: [u8; 1070] = include!("WizooIni.in");
        let single_patch = SinglePatch::from_bytes(data[10..].to_vec());  // skip sysex header and checksum
        assert_eq!(single_patch.unwrap().common.name, "WizooIni");
    }
}
