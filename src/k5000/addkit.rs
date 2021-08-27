//! Data model for the "additive kit" used by an ADD source.
//!

use crate::SystemExclusiveData;
use crate::Checksum;
use crate::k5000::formant::FormantFilter;
use crate::k5000::harmonic::Envelope as HarmonicEnvelope;
use crate::k5000::harmonic::Levels;
use crate::k5000::morf::{HarmonicCommon, MorfHarmonic};

/// Number of harmonics.
pub const HARMONIC_COUNT: usize = 64;

/// Number of formant filter bands.
pub const BAND_COUNT: usize = 128;

/// Additive kit.
pub struct AdditiveKit {
    pub common: HarmonicCommon,
    pub morf: MorfHarmonic,
    pub formant_filter: FormantFilter,
    pub levels: Levels,
    pub bands: [u8; BAND_COUNT],
    pub envelopes: Vec::<HarmonicEnvelope>,
}

impl Default for AdditiveKit {
    fn default() -> Self {
        AdditiveKit {
            common: Default::default(),
            morf: Default::default(),
            formant_filter: Default::default(),
            levels: Default::default(),
            bands: [0; BAND_COUNT],
            envelopes: vec![Default::default(); HARMONIC_COUNT],
        }
    }
}

impl AdditiveKit {
    /// Makes a new additive kit with default values.
    pub fn new() -> Self {
        Default::default()
    }
}

impl SystemExclusiveData for AdditiveKit {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset = 0;

        offset = 164;  // FF bands should start here
        let mut bands: [u8; BAND_COUNT] = [0; BAND_COUNT];
        for i in 0..BAND_COUNT {
            bands[i] = data[offset];
            offset += 1;
        }

        let mut envelopes: Vec::<HarmonicEnvelope> = vec![HarmonicEnvelope::new(); HARMONIC_COUNT];
        for _ in 0..HARMONIC_COUNT {
            envelopes.push(HarmonicEnvelope::from_bytes(data[offset..offset + 8].to_vec()));
            offset += 8;
        }

        AdditiveKit {
            common: HarmonicCommon::from_bytes(data[1..6].to_vec()),
            morf: MorfHarmonic::from_bytes(data[6..19].to_vec()),
            formant_filter: FormantFilter::from_bytes(data[19..36].to_vec()),
            levels: Levels::from_bytes(data[36..164].to_vec()),
            bands: bands,
            envelopes: envelopes,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.checksum());
        result.extend(self.common.to_bytes());
        result.extend(self.morf.to_bytes());
        result.extend(self.formant_filter.to_bytes());
        result.extend(self.levels.to_bytes());
        result.extend(self.bands.to_vec());

        for env in self.envelopes.iter() {
            result.extend(env.to_bytes());
        }

        result.push(0);  // "loud sens select"?

        result
    }
}

impl Checksum for AdditiveKit {
    fn checksum(&self) -> u8 {
        // Additive kit checksum:
        // {(HCKIT sum) + (HCcode1 sum) + (HCcode2 sum) + (FF sum) + (HCenv sum) + (loud sense select) + 0xA5} & 0x7F
        let mut total = 0;

        // HCKIT sum:
        let common_data = self.common.to_bytes();
        let mut common_sum: u32 = 0;
        for d in common_data {
            common_sum += (d & 0xff) as u32;
        }

        let morf_data = self.morf.to_bytes();
        for d in morf_data {
            common_sum += (d & 0xff) as u32;
        }

        let ff_data = self.formant_filter.to_bytes();
        for d in ff_data {
            common_sum += (d & 0xff) as u32;
        }

        total += common_sum & 0xff;

        // HCcode1 sum:
        let mut hc1_sum: u32 = 0;
        for h in self.levels.soft.iter() {
            hc1_sum += (h & 0xff) as u32;
        }

        total += hc1_sum;

        let mut hc2_sum: u32 = 0;
        for h in self.levels.loud.iter() {
            hc2_sum += (h & 0xff) as u32;
        }
        total += hc2_sum;

        // FF sum:
        let mut ff_sum: u32 = 0;
        for f in self.bands.iter() {
            ff_sum += (f & 0xff) as u32;
        }

        total += ff_sum;

        // HCenv sum:
        let mut hcenv_sum: u32 = 0;
        for env in self.envelopes.iter() {
            let ed = env.to_bytes();
            for e in ed {
                hcenv_sum += (e & 0xff) as u32;
            }
        }

        total += hcenv_sum & 0xff;
        total += 0xa5;

        (total & 0x7f) as u8
    }
}
