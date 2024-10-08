//! Data model for patch bank.
//!

use std::fmt;
use log::debug;

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k4::single::SinglePatch;
use crate::k4::multi::MultiPatch;
use crate::k4::effect::EffectPatch;
use crate::k4::drum::DrumPatch;

pub const SINGLE_PATCH_COUNT: usize = 64;  // number of single patches in a bank
pub const MULTI_PATCH_COUNT: usize = 64;   // number of multi patches in a bank
pub const EFFECT_PATCH_COUNT: usize = 32;  // number of effect patches in a bank

pub struct Bank {
    pub singles: Vec<SinglePatch>,
    pub multis: Vec<MultiPatch>,
    pub drum: DrumPatch,
    pub effects: Vec<EffectPatch>,
}

impl Bank {
    fn new() -> Self {
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

        debug!("Parsing single patches, offset = {}", offset);

        let mut singles = Vec::<SinglePatch>::new();
        for i in 0..SINGLE_PATCH_COUNT {
            let single = SinglePatch::from_bytes(&data[offset..]);
            debug!("{}: {}", i, single.as_ref().unwrap().name);
            offset += SinglePatch::data_size();
            singles.push(single?);
        }

        let mut total = 0;
        let mut block_size = SinglePatch::data_size() * SINGLE_PATCH_COUNT;
        total += block_size;

        assert_eq!(offset, total);

        debug!("Parsing multi patches, offset = {}", offset);

        let mut multis = Vec::<MultiPatch>::new();
        for i in 0..MULTI_PATCH_COUNT {
            let multi = MultiPatch::from_bytes(&data[offset..]);
            debug!("{}: {}", i, multi.as_ref().unwrap().name);
            offset += MultiPatch::data_size();
            multis.push(multi?);
        }

        block_size = MultiPatch::data_size() * MULTI_PATCH_COUNT;
        total += block_size;
        assert_eq!(offset, total);

        debug!("Parsing drum patches, offset = {}", offset);

        let drum = DrumPatch::from_bytes(&data[offset..]);
        offset += DrumPatch::data_size();

        block_size = DrumPatch::data_size();
        total += block_size;
        assert_eq!(offset, total);

        debug!("Parsing effect patches, offset = {}", offset);

        let mut effects = Vec::<EffectPatch>::new();
        for i in 0..EFFECT_PATCH_COUNT {
            let effect = EffectPatch::from_bytes(&data[offset..]);
            debug!("{}: {}", i, effect.as_ref().unwrap().effect);
            offset += EffectPatch::data_size();
            effects.push(effect?);
        }

        block_size = EffectPatch::data_size() * EFFECT_PATCH_COUNT;
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
        SinglePatch::data_size() * SINGLE_PATCH_COUNT
        + MultiPatch::data_size() * MULTI_PATCH_COUNT
        + DrumPatch::data_size()
        + EffectPatch::data_size() * EFFECT_PATCH_COUNT
     }
}

#[cfg(test)]
mod tests {
    use super::{*};
    use crate::k4::sysex::Header;

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_bank_from_bytes() {
        let start = 2 + Header::data_size();
        let bank = Bank::from_bytes(&DATA[start..]);

        assert_eq!(bank.as_ref().unwrap().singles.len(), SINGLE_PATCH_COUNT);
        assert_eq!(bank.as_ref().unwrap().effects.len(), EFFECT_PATCH_COUNT);
    }
}
