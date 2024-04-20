//! Data model for the "additive kit" used by an ADD source.
//!

use crate::{
    SystemExclusiveData, 
    ParseError, 
    Checksum
};
use crate::k5000::formant::FormantFilter;
use crate::k5000::harmonic::{
    Envelope as HarmonicEnvelope,
    Levels
};
use crate::k5000::morf::{
    HarmonicCommon, 
    MorfHarmonic
};

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
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset = 0;
        let checksum = data[offset];
        eprintln!("{:#04X}: additive kit checksum = {:#02x}", offset, checksum);
        offset += 1;

        let hc_data = &data[1..7];
        let common = HarmonicCommon::from_bytes(hc_data)?;
        eprintln!("{:#04X}: harmonic common = {}", offset, common);
        offset += common.data_size();

        let morf_data = &data[7..20];
        let morf = MorfHarmonic::from_bytes(morf_data)?;
        eprintln!("{:#04X}: MORF harmonic = {}", offset, morf);
        offset += morf.data_size();

        let ff_data = &data[20..37];
        let formant_filter = FormantFilter::from_bytes(ff_data)?;
        eprintln!("{:#04X}: FF = {}", offset, formant_filter);
        offset += formant_filter.data_size();

        let levels_data = &data[37..165];
        let levels = Levels::from_bytes(levels_data)?;
        offset += levels.data_size();

        eprintln!("{:#04X}: FF bands start here", offset);
        let mut bands: [u8; BAND_COUNT] = [0; BAND_COUNT];
        for i in 0..BAND_COUNT {
            bands[i] = data[offset];
            offset += 1;
        }

        eprintln!("{:#04X}: Harmonic envelopes start here", offset);
        let mut envelopes: Vec::<HarmonicEnvelope> = vec![HarmonicEnvelope::new(); HARMONIC_COUNT];
        for _ in 0..HARMONIC_COUNT {
            envelopes.push(HarmonicEnvelope::from_bytes(&data[offset..offset + 8])?);
            offset += 8;
        }

        Ok(AdditiveKit {
            common,
            morf,
            formant_filter,
            levels,
            bands,
            envelopes,
        })
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

    fn data_size(&self) -> usize {
        1  // checksum

        // HC kit
        + self.common.data_size()
        + self.morf.data_size()
        + self.formant_filter.data_size()

        // HC code 1 & 2
        + self.levels.data_size()

        + BAND_COUNT

        + HARMONIC_COUNT * self.envelopes[0].data_size()
    }
}

impl Checksum for AdditiveKit {
    fn checksum(&self) -> u8 {
        // Additive kit checksum:
        // {(HCKIT sum) + (HCcode1 sum) + (HCcode2 sum) 
        // + (FF sum) + (HCenv sum) + (loud sens select) + 0xA5} & 0x7F
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
