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
        // TODO: parse the bank bytes
        Default::default()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        // TODO: emit the bank bytes
        buf
    }

    fn data_size(&self) -> usize { 8 }
}
