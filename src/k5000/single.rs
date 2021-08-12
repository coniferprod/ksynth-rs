//! Data model for single patches.
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
use crate::k5000::{RangedValue, RangeKind};

/// Single patch common data.
pub struct Common {
    pub name: String,
    pub volume: RangedValue,
    pub polyphony: Polyphony,
    pub source_count: u8,
    pub source_mutes: [bool; 6],
    pub portamento_active: bool,
    pub portamento_speed: RangedValue,
    pub amplitude_modulation: AmplitudeModulation,
    pub macros: [MacroController; 4],
    pub switches: SwitchControl,
    pub effects: EffectSettings,
    pub geq: [i8; 7],
    pub effect_control: EffectControl,
}

impl Default for Common {
    fn default() -> Self {
        Common {
            name: "NewSound".to_string(),
            volume: RangedValue::from_int(RangeKind::PositiveLevel, 99),
            polyphony: Polyphony::Poly,
            source_count: 2,
            source_mutes: [false, false, true, true, true, true],
            portamento_active: false,
            portamento_speed: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            amplitude_modulation: Default::default(),
            macros: [Default::default(), Default::default(), Default::default(), Default::default()],
            switches: Default::default(),
            effects: Default::default(),
            geq: [0, 0, 0, 0, 0, 0, 0],
            effect_control: Default::default(),
        }
    }
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn vec_to_array(v: Vec<i8>) -> [i8; 7] {
    v.try_into()
        .unwrap_or_else(|v: Vec<i8>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()))
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

impl SystemExclusiveData for Common {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("SingleCommon");

        let effects = EffectSettings::from_bytes(data[..31].to_vec());

        let geq_values = data[31..38].to_vec().iter().map(|n| (n - 64) as i8).collect();  // 58(-6) ~ 70(+6), so 64 is zero

        let name = String::from_utf8(data[38..46].to_vec()).unwrap();
        eprintln!("Name = {}", name);

        let mutes_byte = data[50];
        let mut mutes: [bool; 6] = [false; 6];
        for i in 0..6 {
            mutes[i] = get_bit_at(mutes_byte as u32, i as u8);
        }

        let mut offset = 60;
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
                depth1: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[0]),
                destination2: ControlDestination::try_from(macro_destinations[1]).unwrap(),
                depth2: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[1]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[2]).unwrap(),
                depth1: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[2]),
                destination2: ControlDestination::try_from(macro_destinations[3]).unwrap(),
                depth2: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[3]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[4]).unwrap(),
                depth1: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[4]),
                destination2: ControlDestination::try_from(macro_destinations[5]).unwrap(),
                depth2: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[5]),
            },

            MacroController {
                destination1: ControlDestination::try_from(macro_destinations[6]).unwrap(),
                depth1: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[6]),
                destination2: ControlDestination::try_from(macro_destinations[7]).unwrap(),
                depth2: RangedValue::from_byte(RangeKind::MacroDepth, macro_depths[7]),
            },
        ];

        Common {
            effects: effects,
            geq: vec_to_array(geq_values),
            name: name,
            volume: RangedValue::from_byte(RangeKind::PositiveLevel, data[46]),
            polyphony: Polyphony::try_from(data[47]).unwrap(),
            source_count: data[49],
            source_mutes: mutes,
            amplitude_modulation: AmplitudeModulation::try_from(data[51]).unwrap(),
            effect_control: EffectControl::from_bytes(data[52..58].to_vec()),
            portamento_active: if data[58] == 1 { true } else { false },
            portamento_speed: RangedValue::from_byte(RangeKind::PositiveLevel, data[59]),
            macros: macros,
            switches: SwitchControl {
                switch1: Switch::try_from(data[76]).unwrap(),
                switch2: Switch::try_from(data[77]).unwrap(),
                footswitch1: Switch::try_from(data[78]).unwrap(),
                footswitch2: Switch::try_from(data[79]).unwrap(),
            },
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.effects.to_bytes());
        result.extend(self.geq.to_vec().iter().map(|n| (n + 64) as u8));
        result.push(0);  // drum_mark
        result.extend(self.name.clone().into_bytes());  // note clone()
        result.push(self.volume.as_byte());
        result.push(self.polyphony as u8);
        result.push(0);
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
        result.push(if self.portamento_active { 1 } else { 0 });
        result.push(self.portamento_speed.as_byte());

        // Pick out the destinations and depths as the SysEx spec wants them.
        for m in &self.macros {
            result.push(m.destination1 as u8);
            result.push(m.destination2 as u8);
        }

        for m in &self.macros {
            result.push(m.depth1.as_byte()); // -31(33)~+31(95)
            result.push(m.depth2.as_byte());
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
        for i in 0..common.source_count {
            let source = Source::from_bytes(data[offset..offset + 86].to_vec());
            sources.push(source);
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

        let mut total = 0;
        let mut count = 0;

        let common_data = self.common.to_bytes();
        let mut common_sum: u32 = 0;
        for d in common_data.iter() {
            common_sum += (d & 0xff) as u32;
            count += 1;
        }

        total += common_sum & 0xff;

        for source in self.sources.iter() {
            let mut source_sum = 0;
            let source_data = source.to_bytes();
            for d in source_data.iter() {
                source_sum = d & 0xff;
                count += 1;
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
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset = 0;

        let original_checksum = data[0];
        eprintln!("original checksum = {:#02x}", original_checksum);

        let common = Common::from_bytes(data[1..82].to_vec());
        offset += 81;
        let mut sources = Vec::<Source>::new();
        for i in 0..common.source_count {
            let source = Source::from_bytes(data[offset..offset + 86].to_vec());
            sources.push(source);
            offset += 86;
        }

        let mut additive_kits = BTreeMap::<String, AdditiveKit>::new();

        // How many additive kits should we expect then?
        let kit_count = sources.iter().filter(|s| s.oscillator.wave == 512).count();
        let mut kit_index = 0;
        while kit_index < kit_count {
            let kit = AdditiveKit::from_bytes(data[offset..offset + 806].to_vec());
            offset += 806;
            let kit_name = format!("s{}", kit_index + 1);
            additive_kits.insert(kit_name, kit);
            kit_index += 1;
        }

        SinglePatch {
            common: common,
            sources: sources,
            additive_kits: additive_kits,
        }
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
        write!(f, "{}", self.common)
    }
}
