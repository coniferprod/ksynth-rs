//! Data model for the source in a single patch.
//!

use crate::k5000::control::{VelocitySwitchSettings, ModulationSettings, PanSettings};
use crate::SystemExclusiveData;
use crate::k5000::osc::*;
use crate::k5000::filter::*;
use crate::k5000::amp::*;
use crate::k5000::lfo::*;
use crate::k5000::{UnsignedLevel, UnsignedCoarse, MediumDepth};

/// Key in a keyboard zone.
pub struct Key {
    /// MIDI note number for the key.
    pub note: u8,
}

impl Key {
    // TODO: Add constructor from note name

    pub fn name(&self) -> String {
        let note_names = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = self.note / 12 - 1;
        let name = note_names[(self.note as usize) % 12];
        format!("{}{}", name, octave)
    }
}

/// Keyboard zone.
pub struct Zone {
    /// Low key of the zone.
    pub low: Key,

    /// High key of the zone.
    pub high: Key,
}

impl SystemExclusiveData for Zone {
    fn from_bytes(data: Vec<u8>) -> Self {
        Zone { low: Key { note: data[0] }, high: Key { note: data[1] } }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.low.note);
        result.push(self.high.note);

        result
    }
}

/// Source control settings.
pub struct SourceControl {
    pub zone: Zone,
    pub vel_sw: VelocitySwitchSettings,
    pub effect_path: u8,
    pub volume: UnsignedLevel,
    pub bender_pitch: UnsignedCoarse,
    pub bender_cutoff: MediumDepth,
    pub modulation: ModulationSettings,
    pub key_on_delay: UnsignedLevel,
    pub pan: PanSettings,
}

impl Default for SourceControl {
    fn default() -> Self {
        SourceControl {
            zone: Zone { low: Key { note: 0 }, high: Key { note: 127 } },
            vel_sw: Default::default(),
            effect_path: 0,
            volume: UnsignedLevel::from(100),
            bender_pitch: UnsignedCoarse::from(0),
            bender_cutoff: MediumDepth::from(0),
            modulation: Default::default(),
            key_on_delay: UnsignedLevel::from(0),
            pan: Default::default(),
        }
    }
}

impl SystemExclusiveData for SourceControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        SourceControl {
            zone: Zone { low: Key { note: data[0] }, high: Key { note: data[1] } },
            vel_sw: VelocitySwitchSettings::from_bytes(vec![data[2]]),
            effect_path: data[3],
            volume: UnsignedLevel::from(data[4]),
            bender_pitch: UnsignedCoarse::from(data[5]),
            bender_cutoff: MediumDepth::from(data[6]),
            modulation: ModulationSettings::from_bytes(data[7..25].to_vec()),
            key_on_delay: UnsignedLevel::from(data[25]),
            pan: PanSettings::from_bytes(data[26..28].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.zone.to_bytes());
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
