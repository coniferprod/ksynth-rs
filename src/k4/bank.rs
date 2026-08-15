//! Data model for patch bank.
//!

use std::fmt;

use log;
use syxpack::{
    SystemExclusiveData,
    ParseError
};

use crate::k4::{drum, single, multi, effect};

/// Number of single patches in a bank.
pub const SINGLE_PATCH_COUNT: usize = 64;

/// Number of multi patches in a bank.
pub const MULTI_PATCH_COUNT: usize = 64;

/// Number of effect patches in a bank
pub const EFFECT_PATCH_COUNT: usize = 32;  

/// Single patch data.
type SingleData = [u8; single::DATA_SIZE];

/// Multi patch data.
type MultiData = [u8; multi::DATA_SIZE];

/// Drum patch data.
type DrumData = [u8; drum::DATA_SIZE];


/// Effect patch data.
type EffectData = [u8; effect::DATA_SIZE];

/// Bank as raw data.
pub struct BankData {
    singles: [SingleData; SINGLE_PATCH_COUNT],
    multis: [MultiData; MULTI_PATCH_COUNT],
    drum: DrumData,
    effects: [EffectData; EFFECT_PATCH_COUNT],
}

impl BankData {
    /// Make a new, empty bank with all zeros.
    pub fn new() -> BankData {
        Self {
            singles: [[0u8; single::DATA_SIZE]; SINGLE_PATCH_COUNT],
            multis: [[0u8; multi::DATA_SIZE]; MULTI_PATCH_COUNT],
            drum: [0u8; drum::DATA_SIZE],
            effects: [[0u8; effect::DATA_SIZE]; EFFECT_PATCH_COUNT],
        }
    }

    pub fn put_single(&mut self, data: &SingleData, slot: usize) {
        assert!(slot < SINGLE_PATCH_COUNT);

        self.singles[slot].copy_from_slice(data);
    }

    pub fn put_multi(&mut self, data: &MultiData, slot: usize) {
        assert!(slot < MULTI_PATCH_COUNT);

        self.multis[slot].copy_from_slice(data);
    }

    pub fn put_drum(&mut self, data: &DrumData) {
        self.drum.copy_from_slice(data);
    }

    pub fn put_effect(&mut self, data: &EffectData, slot: usize) {
        assert!(slot < EFFECT_PATCH_COUNT);

        self.effects[slot].copy_from_slice(data);
    }

    pub fn get_single(&self, slot: usize) -> SingleData {
        assert!(slot <= SINGLE_PATCH_COUNT);

        self.singles[slot]
    }

    pub fn get_multi(&self, slot: usize) -> MultiData {
        assert!(slot <= MULTI_PATCH_COUNT);

        self.multis[slot]
    }

    pub fn get_drum(&self) -> DrumData {
        self.drum
    }

    pub fn get_effect(&self, slot: usize) -> EffectData {
        assert!(slot <= EFFECT_PATCH_COUNT);

        self.effects[slot]
    }
}

impl SystemExclusiveData for BankData {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut result = BankData::new();

        let mut offset = 0;

        for i in 0..SINGLE_PATCH_COUNT {
            result.singles[i].copy_from_slice(&data[offset..offset + single::DATA_SIZE]);
            offset += single::DATA_SIZE;
        }

        for i in 0..MULTI_PATCH_COUNT {
            result.multis[i].copy_from_slice(&data[offset..offset + multi::DATA_SIZE]);
            offset += multi::DATA_SIZE;
        }

        result.drum.copy_from_slice(&data[offset..offset + drum::DATA_SIZE]);
        offset += drum::DATA_SIZE;

        for i in 0..EFFECT_PATCH_COUNT {
            result.effects[i].copy_from_slice(&data[offset..offset + effect::DATA_SIZE]);
            offset += effect::DATA_SIZE;
        }

        Ok(result)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for i in 0..SINGLE_PATCH_COUNT {
            buf.extend(self.singles[i].to_vec());
        }

        for i in 0..MULTI_PATCH_COUNT {
            buf.extend(self.multis[i].to_vec());
        }

        buf.extend(self.drum.to_vec());

        for i in 0..EFFECT_PATCH_COUNT {
            buf.extend(self.effects[i].to_vec());
        }

        // full bank minus SysEx header and terminator
        assert_eq!(buf.len(), 15123 - 8 - 1);

        buf
    }

    fn data_size() -> usize {
        single::DATA_SIZE * SINGLE_PATCH_COUNT
        + multi::DATA_SIZE * MULTI_PATCH_COUNT
        + drum::DATA_SIZE
        + effect::DATA_SIZE * EFFECT_PATCH_COUNT
    }
}

pub struct Bank {
    pub singles: Vec<single::SinglePatch>,
    pub multis: Vec<multi::MultiPatch>,
    pub drum: drum::DrumPatch,
    pub effects: Vec<effect::EffectPatch>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            singles: vec![Default::default(); SINGLE_PATCH_COUNT],
            multis: vec![Default::default(); MULTI_PATCH_COUNT],
            drum: Default::default(),
            effects: vec![Default::default(); EFFECT_PATCH_COUNT],
        }
    }
}

impl Default for Bank {
    fn default() -> Self {
        Bank::new()
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
            "SINGLES:\n\nMULTIS:\n\nDRUM:\n\nEFFECTS:\n...later..."
        )
    }
}

impl SystemExclusiveData for Bank {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset = 0;

        log::debug!("Parsing single patches, offset = {}", offset);

        let mut singles = Vec::<single::SinglePatch>::new();
        for i in 0..SINGLE_PATCH_COUNT {
            let single = single::SinglePatch::from_bytes(&data[offset..]);
            log::debug!("{}: {}", i, single.as_ref().unwrap().name);
            offset += single::SinglePatch::data_size();
            singles.push(single?);
        }

        let mut total = 0;
        let mut block_size = single::SinglePatch::data_size() * SINGLE_PATCH_COUNT;
        total += block_size;

        assert_eq!(offset, total);

        log::debug!("Parsing multi patches, offset = {}", offset);

        let mut multis = Vec::<multi::MultiPatch>::new();
        for i in 0..MULTI_PATCH_COUNT {
            let multi = multi::MultiPatch::from_bytes(&data[offset..]);
            log::debug!("{}: {}", i, multi.as_ref().unwrap().name);
            offset += multi::MultiPatch::data_size();
            multis.push(multi?);
        }

        block_size = multi::MultiPatch::data_size() * MULTI_PATCH_COUNT;
        total += block_size;
        assert_eq!(offset, total);

        log::debug!("Parsing drum patches, offset = {}", offset);

        let drum = drum::DrumPatch::from_bytes(&data[offset..]);
        offset += drum::DrumPatch::data_size();

        block_size = drum::DrumPatch::data_size();
        total += block_size;
        assert_eq!(offset, total);

        log::debug!("Parsing effect patches, offset = {}", offset);

        let mut effects = Vec::<effect::EffectPatch>::new();
        for i in 0..EFFECT_PATCH_COUNT {
            let effect = effect::EffectPatch::from_bytes(&data[offset..]);
            log::debug!("{}: {}", i, effect.as_ref().unwrap().effect);
            offset += effect::EffectPatch::data_size();
            effects.push(effect?);
        }

        block_size = effect::EffectPatch::data_size() * EFFECT_PATCH_COUNT;
        total += block_size;
        assert_eq!(offset, total);

        Ok(Bank {
            singles,
            multis,
            drum: drum.unwrap(),
            effects,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for i in 0..SINGLE_PATCH_COUNT {
            buf.extend(self.singles[i].to_bytes());
        }

        for i in 0..MULTI_PATCH_COUNT {
            buf.extend(self.multis[i].to_bytes());
        }

        buf.extend(self.drum.to_bytes());

        for i in 0..EFFECT_PATCH_COUNT {
            buf.extend(self.effects[i].to_bytes());
        }

        // full bank minus SysEx header and terminator
        assert_eq!(buf.len(), 15123 - 8 - 1);

        buf
    }

    fn data_size() -> usize {
        single::DATA_SIZE * SINGLE_PATCH_COUNT
        + multi::DATA_SIZE * MULTI_PATCH_COUNT
        + drum::DATA_SIZE
        + effect::DATA_SIZE * EFFECT_PATCH_COUNT
    }
}

#[cfg(test)]
mod tests {
    use super::{*};
    use crate::k4::sysex::Header;

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_bank_from_bytes() {
        let start = 2 + Header::data_size();  // skip F0 40
        let bank = Bank::from_bytes(&DATA[start..]);

        assert_eq!(bank.as_ref().unwrap().singles.len(), SINGLE_PATCH_COUNT);
        assert_eq!(bank.as_ref().unwrap().effects.len(), EFFECT_PATCH_COUNT);
    }

    /*
    #[test]
    fn test_bank_data_from_bytes() {
        let start = 2 + Header::data_size();  // skip F0 40
        let bank = Bank::data_from_bytes(&DATA[start..]);


    }
     */
}
