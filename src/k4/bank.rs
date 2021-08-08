use std::convert::TryFrom;
use std::convert::TryInto;
use num_enum::TryFromPrimitive;
use std::fmt;
use crate::k4::SUBMIX_COUNT;
use crate::{SystemExclusiveData, Checksum};
use crate::k4::single::SinglePatch;
use crate::k4::multi::MultiPatch;
use crate::k4::effect::EffectPatch;
use crate::k4::drum::DrumPatch;

const SINGLE_PATCH_COUNT: usize = 64;  // number of single patches in a bank
const MULTI_PATCH_COUNT: usize = 64;   // number of multi patches in a bank
const EFFECT_PATCH_COUNT: usize = 32;  // number of effect patches in a bank

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
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset = 0;

        eprintln!("Parsing single patches, offset = {}", offset);

        let mut singles = Vec::<SinglePatch>::new();
        for i in 0..SINGLE_PATCH_COUNT {
            let single = SinglePatch::from_bytes(data[offset..].to_vec());
            eprintln!("{}: {}", i, single.name);
            offset += single.data_size();
            singles.push(single);
        }

        let mut total = 0;
        let mut block_size = singles[0].data_size() * SINGLE_PATCH_COUNT;
        total += block_size;

        assert_eq!(offset, total);

        eprintln!("Parsing multi patches, offset = {}", offset);

        let mut multis = Vec::<MultiPatch>::new();
        for i in 0..MULTI_PATCH_COUNT {
            let multi = MultiPatch::from_bytes(data[offset..].to_vec());
            eprintln!("{}: {}", i, multi.name);
            offset += multi.data_size();
            multis.push(multi);
        }

        block_size = multis[0].data_size() * MULTI_PATCH_COUNT;
        total += block_size;
        assert_eq!(offset, total);

        eprintln!("Parsing drum patches, offset = {}", offset);

        let drum = DrumPatch::from_bytes(data[offset..].to_vec());
        offset += drum.data_size();

        block_size = drum.data_size();
        total += block_size;
        assert_eq!(offset, total);

        eprintln!("Parsing effect patches, offset = {}", offset);

        let mut effects = Vec::<EffectPatch>::new();
        for i in 0..EFFECT_PATCH_COUNT {
            let effect = EffectPatch::from_bytes(data[offset..].to_vec());
            eprintln!("{}: {}", i, effect.effect);
            offset += effect.data_size();
            effects.push(effect);
        }

        block_size = effects[0].data_size() * EFFECT_PATCH_COUNT;
        total += block_size;
        assert_eq!(offset, total);

        Bank {
            singles,
            multis,
            drum,
            effects,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        // TODO: emit the bank bytes
        buf
    }

    fn data_size(&self) -> usize {
        self.singles[0].data_size() * SINGLE_PATCH_COUNT
        + self.multis[0].data_size() * MULTI_PATCH_COUNT
        + self.drum.data_size()
        + self.effects[0].data_size() * EFFECT_PATCH_COUNT
     }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_bank_from_bytes() {
        let data: [u8; 15123] = include!("a401.in");

        // Skip the SysEx header when constructing the bank
        let bank = Bank::from_bytes(data[8..].to_vec());

        assert_eq!(bank.singles.len(), SINGLE_PATCH_COUNT);
        assert_eq!(bank.effects.len(), EFFECT_PATCH_COUNT);
    }
}
