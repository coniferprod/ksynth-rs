//! Data model for the source in a single patch.
//!

use crate::k5000::control::{VelocitySwitchSettings, ModulationSettings, PanSettings};
use crate::SystemExclusiveData;
use crate::k5000::osc::*;
use crate::k5000::filter::*;
use crate::k5000::amp::*;
use crate::k5000::lfo::*;
use crate::k5000::{RangedValue, RangeKind};

/// Source control settings.
pub struct SourceControl {
    zone_low: RangedValue,
    zone_high: RangedValue,
    vel_sw: VelocitySwitchSettings,
    effect_path: u8,
    volume: RangedValue,
    bender_pitch: RangedValue,
    bender_cutoff: RangedValue,
    modulation: ModulationSettings,
    key_on_delay: RangedValue,
    pan: PanSettings,
}

impl Default for SourceControl {
    fn default() -> Self {
        SourceControl {
            zone_low: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            zone_high: RangedValue::from_int(RangeKind::PositiveLevel, 127),
            vel_sw: Default::default(),
            effect_path: 0,
            volume: RangedValue::from_int(RangeKind::PositiveLevel, 100),
            bender_pitch: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            bender_cutoff: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            modulation: Default::default(),
            key_on_delay: RangedValue::from_int(RangeKind::PositiveLevel, 0),
            pan: Default::default(),
        }
    }
}

impl SystemExclusiveData for SourceControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        SourceControl {
            zone_low: RangedValue::from_byte(RangeKind::PositiveLevel, data[0]),
            zone_high: RangedValue::from_byte(RangeKind::PositiveLevel, data[1]),
            vel_sw: VelocitySwitchSettings::from_bytes(vec![data[2]]),
            effect_path: data[3],
            volume: RangedValue::from_byte(RangeKind::PositiveLevel, data[4]),
            bender_pitch: RangedValue::from_byte(RangeKind::PositiveLevel, data[5]),
            bender_cutoff: RangedValue::from_byte(RangeKind::PositiveLevel, data[6]),
            modulation: ModulationSettings::from_bytes(data[7..25].to_vec()),
            key_on_delay: RangedValue::from_byte(RangeKind::PositiveLevel, data[25]),
            pan: PanSettings::from_bytes(data[26..28].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.zone_low.as_byte());
        result.push(self.zone_high.as_byte());
        result.extend(self.vel_sw.to_bytes());
        result.push(self.effect_path);
        result.push(self.volume.as_byte());
        result.push(self.bender_pitch.as_byte());
        result.push(self.bender_cutoff.as_byte());
        result.extend(self.modulation.to_bytes());
        result.push(self.key_on_delay.as_byte());
        result.extend(self.pan.to_bytes());

        result
    }
}

/// Source.
pub struct Source {
    pub oscillator: Oscillator,
    pub filter: Filter,
    pub amplifier: Amplifier,
    pub lfo: Lfo,
    pub control: SourceControl,
}

impl Source {
    /// Makes a new PCM source with default values.
    pub fn pcm() -> Source {
        Default::default()
    }

    /// Returns `true` if this source is ADD, false if PCM.
    pub fn is_additive(&self) -> bool {
        self.oscillator.wave == 512
    }

    /// Returns `true` if this source is PCM, false if ADD.
    pub fn is_pcm(&self) -> bool {
        !self.is_additive()
    }

    /// Makes a new ADD source with default values.
    pub fn additive() -> Source {
        Source {
            oscillator: Oscillator::additive(),
            filter: Default::default(),
            amplifier: Default::default(),
            lfo: Default::default(),
            control: Default::default(),
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source {
            oscillator: Default::default(),
            filter: Default::default(),
            amplifier: Default::default(),
            lfo: Default::default(),
            control: Default::default(),
        }
    }
}

impl SystemExclusiveData for Source {
    fn from_bytes(data: Vec<u8>) -> Self {
        Source {
            control: SourceControl::from_bytes(data[..28].to_vec()),
            oscillator: Oscillator::from_bytes(data[28..40].to_vec()),
            filter: Filter::from_bytes(data[40..60].to_vec()),
            amplifier: Amplifier::from_bytes(data[60..75].to_vec()),
            lfo: Lfo::from_bytes(data[75..86].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.control.to_bytes());
        result.extend(self.oscillator.to_bytes());
        result.extend(self.filter.to_bytes());
        result.extend(self.amplifier.to_bytes());
        result.extend(self.lfo.to_bytes());

        result
    }
}
