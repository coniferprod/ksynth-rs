//! Data model for the source in a single patch.
//!

use std::fmt;

use crate::k5000::control::{
    VelocitySwitchSettings,
    ModulationSettings,
    PanSettings
};
use crate::{
    SystemExclusiveData,
    ParseError,
    Ranged
};
use crate::k5000::osc::*;
use crate::k5000::filter::*;
use crate::k5000::amp::*;
use crate::k5000::lfo::*;
use crate::k5000::{
    Volume,
    BenderPitch,
    BenderCutoff,
    KeyOnDelay
};

use pretty_hex::*;

/// Key in a keyboard zone.
#[derive(Debug, Eq, PartialEq)]
pub struct Key {
    /// MIDI note number for the key.
    pub note: u8,
}

static NOTE_NAMES: &str = "C C#D D#E F F#G G#A A#B ";

impl Key {
    // TODO: Add constructor from note name

    pub fn name(&self) -> String {
        // Adapted from RIMD:
        let octave = (self.note as f32 / 12 as f32).floor() - 1.0;
        let name_index = (self.note as usize % 12) * 2;
        let slice = if NOTE_NAMES.as_bytes()[name_index + 1] == ' ' as u8 {
            &NOTE_NAMES[name_index..(name_index + 1)]
        } else {
            &NOTE_NAMES[name_index..(name_index + 2)]
        };
        format!("{}{}", slice, octave)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Keyboard zone.
#[derive(Debug)]
pub struct Zone {
    /// Low key of the zone.
    pub low: Key,

    /// High key of the zone.
    pub high: Key,
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {}", self.low, self.high)
    }
}

impl Default for Zone {
    fn default() -> Self {
        Zone { low: Key { note: 0 }, high: Key { note: 127 } }
    }
}

impl SystemExclusiveData for Zone {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Zone { low: Key { note: data[0] }, high: Key { note: data[1] } })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.low.note,
            self.high.note,
        ]
    }

    fn data_size() -> usize { 2 }
}

/// Source control settings.
#[derive(Debug)]
pub struct SourceControl {
    pub zone: Zone,
    pub vel_sw: VelocitySwitchSettings,
    pub effect_path: u8,
    pub volume: Volume,
    pub bender_pitch: BenderPitch,
    pub bender_cutoff: BenderCutoff,
    pub modulation: ModulationSettings,
    pub key_on_delay: KeyOnDelay,
    pub pan: PanSettings,
}

impl Default for SourceControl {
    fn default() -> Self {
        SourceControl {
            zone: Default::default(),
            vel_sw: Default::default(),
            effect_path: 0,
            volume: Volume::new(100),
            bender_pitch: Default::default(),
            bender_cutoff: Default::default(),
            modulation: Default::default(),
            key_on_delay: Default::default(),
            pan: Default::default(),
        }
    }
}

impl fmt::Display for SourceControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zone={}\nVel. switch: {}\nEffect Path={}\nVolume={}\nBender: Pitch={} Cutoff={}\nKey On Delay={}\nPan: Type={} Value={}\n",
            self.zone, self.vel_sw, self.effect_path, self.volume, self.bender_pitch, self.bender_cutoff, self.key_on_delay, self.pan.pan_type, self.pan.pan_value
        )
    }
}

impl SystemExclusiveData for SourceControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("Source control data = {}", simple_hex(&data));

        Ok(SourceControl {
            zone: Zone { low: Key { note: data[0] }, high: Key { note: data[1] } },
            vel_sw: VelocitySwitchSettings::from_bytes(&[data[2]])?,
            effect_path: data[3],
            volume: Volume::new(data[4] as i32),
            bender_pitch: BenderPitch::new(data[5] as i32),
            bender_cutoff: BenderCutoff::new(data[6] as i32),
            modulation: ModulationSettings::from_bytes(&data[7..25])?,
            key_on_delay: KeyOnDelay::from(data[25]),
            pan: PanSettings::from_bytes(&data[26..28])?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.zone.to_bytes());
        result.extend(self.vel_sw.to_bytes());
        result.push(self.effect_path);
        result.push(self.volume.into());
        result.push(self.bender_pitch.into());
        result.push(self.bender_cutoff.into());
        result.extend(self.modulation.to_bytes());
        result.push(self.key_on_delay.into());
        result.extend(self.pan.to_bytes());

        result
    }

    fn data_size() -> usize {  // should be 28
        Zone::data_size()
        + VelocitySwitchSettings::data_size()
        + 4  // effect path, volume, bender pitch, bender cutoff
        + ModulationSettings::data_size()
        + 1  // key on delay
        + PanSettings::data_size()
    }
}

/// Source.
#[derive(Default, Debug)]
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
        self.oscillator.wave.is_additive()
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

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\nDCO:\n{}\n\nDCF:\n{}\n\nDCA:\n{}\n\nLFO:\n{}\n",
            self.control, self.oscillator, self.filter, self.amplifier, self.lfo)
    }
}

impl SystemExclusiveData for Source {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        //eprintln!("Source data ({} bytes): {:?}", data.len(), data);
        eprintln!("Source data size = {} bytes", data.len());
        eprintln!("Reported sizes:");
        let source_control_size = SourceControl::data_size();
        eprintln!("Source control = {} bytes",
            source_control_size);
        let amplifier_size = Amplifier::data_size();
        eprintln!("Amplifier data = {} bytes",
            amplifier_size);
        let oscillator_size = Oscillator::data_size();
        eprintln!("Oscillator data = {} bytes",
            oscillator_size);
        let filter_size = Filter::data_size();
        eprintln!("Filter data = {} bytes",
            filter_size);
        let lfo_size = Lfo::data_size();
        eprintln!("LFO data = {} bytes",
            lfo_size);
        let total_size = source_control_size + amplifier_size + oscillator_size
            + filter_size + lfo_size;
        eprintln!("Total = {} bytes", total_size);

        Ok(Source {
            control: SourceControl::from_bytes(&data[..28])?,
            oscillator: Oscillator::from_bytes(&data[28..40])?,
            filter: Filter::from_bytes(&data[40..60])?,
            amplifier: Amplifier::from_bytes(&data[60..75])?,
            lfo: Lfo::from_bytes(&data[75..86])?,
        })
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

    fn data_size() -> usize {
        SourceControl::data_size()
        + Oscillator::data_size()
        + Filter::data_size()
        + Amplifier::data_size()
        + Lfo::data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_key_name() {
        let key = Key { note: 60 };
        assert_eq!(key.name(), "C4");
    }

    #[test]
    fn test_source_control_from_bytes() {
        let data = vec![
            0x00, 0x7f,  // zone low and high
            0x10,  // velocity switch
            0x00,  // effect path
            0x78,  // volume
            0x02, 0x0c,  // bender pitch and cutoff
            0x01, 0x4f, 0x03, 0x40,  // press: destination 1, depth, destination 2, depth
            0x03, 0x59, 0x01, 0x40,  // wheel: destination 1, depth, destination 2, depth
            0x02, 0x5f, 0x00, 0x40,  // express: destination 1, depth, destination 2, depth
            0x02, 0x0d, 0x40,  // assignable: control source 1, destination, depth
            0x02, 0x09, 0x40,  // assignable: control source 2, destination, depth
            0x00,  // key on delay
            0x00, 0x40,  // pan type and value
        ];

        let source_control = SourceControl::from_bytes(&data);
        assert_eq!(source_control.as_ref().unwrap().zone.low.note, 0x00);
        assert_eq!(source_control.as_ref().unwrap().zone.high.note, 0x7f);
        assert_eq!(source_control.as_ref().unwrap().volume.value(), 0x78);
    }

    #[test]
    fn test_source_from_bytes() {
        let data = vec![
            0x00, 0x7f,  // zone low and high
            0x10,  // velocity switch
            0x00,  // effect path
            0x78,  // volume
            0x02, 0x0c,  // bender pitch and cutoff
            0x01, 0x4f, 0x03, 0x40,  // press: destination 1, depth, destination 2, depth
            0x03, 0x59, 0x01, 0x40,  // wheel: destination 1, depth, destination 2, depth
            0x02, 0x5f, 0x00, 0x40,  // express: destination 1, depth, destination 2, depth
            0x02, 0x0d, 0x40,  // assignable: control source 1, destination, depth
            0x02, 0x09, 0x40,  // assignable: control source 2, destination, depth
            0x00,  // key on delay
            0x00, 0x40,  // pan type and value
            0x04, 0x00,  // wave kit MSB and LSB
            0x40,  // coarse
            0x40,  // fine
            0x00,  // fixed key
            0x00,  // ks pitch
            0x40, 0x04, 0x40, 0x40, 0x40, 0x40,  // pitch envelope

            // DCF
            0x00,  // 0=active, 1=bypass
            0x00,  // mode: 0=low pass, 1=high pass
            0x04,  // velocity curve
            0x00,  // resonance
            0x00,  // DCF level
            0x37,  // cutoff
            0x60,  // cutoff ks depth
            0x40,  // cutoff velo depth
            0x59,  // DCF env depth
            0x00, 0x78, 0x7f, 0x50, 0x7f, 0x14,  // DCF envelope
            0x40, 0x40,  // DCF KS to Env
            0x5e, 0x40, 0x40,  // DCF Vel to Env

            // DCA
            0x04,  // vel curve
            0x01, 0x5e, 0x7f, 0x3f, 0x7f, 0x0f,  // DCA env
            0x40, 0x40, 0x40, 0x40,  // DCA KS to Env
            0x14, 0x40, 0x40, 0x40,  // DCA Vel Sense
            0x00,  // LFO waveform
            0x5d,  // LFO speed
            0x00,  // LFO delay onset
            0x00,  //  fade in time
            0x00,  //  fade in to speed
            0x00, 0x40,  // pitch (vibrato) depth and KS
            0x00, 0x40,  // DCF (growl) depth and KS
            0x00, 0x40,  // DCA (tremolo) depth and KS
        ];

        let source = Source::from_bytes(&data);
        assert_eq!(source.unwrap().lfo.speed.value(), 0x5d);
    }
}
