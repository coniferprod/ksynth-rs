//! Data model for drum patches.
//!

use std::convert::TryFrom;
use std::fmt;

use log::debug;
use bit::BitIndex;
use syxpack::{
    SystemExclusiveData,
    ParseError,
    MidiChannel,
    Ranged,
    Encoding,
    parse_or_default,
};

use crate::Checksum;
use crate::k4::{
    DRUM_NOTE_COUNT,
    Level,
    ModulationDepth,
    Decay
};
use crate::k4::wave::Wave;
use crate::k4::effect::Submix;

pub const DATA_SIZE: usize = 131;

pub struct DrumPatch {
    pub common: Common,
    pub notes: [Note; DRUM_NOTE_COUNT],
}

impl Default for DrumPatch {
    fn default() -> Self {
        DrumPatch {
            common: Default::default(),
            notes: [Default::default(); DRUM_NOTE_COUNT],
        }
    }
}

impl DrumPatch {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.extend(self.common.to_bytes()); // includes the common checksum

        for i in 0..DRUM_NOTE_COUNT {
            buf.extend(self.notes[i].to_bytes());  // includes the note checksum
        }

        buf
    }
}

impl fmt::Display for DrumPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut notes_str = String::new();
        for i in 0..DRUM_NOTE_COUNT {
            notes_str.push_str(&format!("{}: {}\n", i, self.notes[i]));
        }

        write!(
            f,
            "COMMON: {}\nNOTES:\n{}",
            self.common, notes_str
        )
    }
}

impl SystemExclusiveData for DrumPatch {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let common = Common::parse(&data[0..]);
        let mut offset = Common::data_size();
        let mut notes = [Default::default(); DRUM_NOTE_COUNT];

        for i in 0..DRUM_NOTE_COUNT {
            debug!("Parsing drum note {}, offset = {}", i, offset);

            let note = Note::parse(&data[offset..])?;
            notes[i] = note;
            offset += Note::data_size();
        }

        Ok(DrumPatch {
            common: common?,
            notes,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        // Note: the drum patch doesn't have an overall checksum
        buf
    }

    fn data_size() -> usize {
        Common::data_size()
            + Note::data_size() * DRUM_NOTE_COUNT
    }
}

pub const COMMON_DATA_SIZE: usize = 11;

/// Drum common data.
pub struct Common {
    pub channel: MidiChannel,  // MIDI channel, here 1...16, stored in SysEx as 0...15
    pub volume: Level, // 0~100
    pub velocity_depth: ModulationDepth,  // 0~100
}

impl Default for Common {
    fn default() -> Self {
        Common {
            channel: MidiChannel::new(10),
            volume: Level::new(100),
            velocity_depth: ModulationDepth::new(0),
        }
    }
}

impl Common {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "channel = {}, volume = {}, vel.depth = {}",
            self.channel,
            self.volume,
            self.velocity_depth
        )
    }
}

impl Common {
    fn collect_data(&self) -> Vec<u8> {
        vec![
            self.channel.encode(),
            self.volume.encode(),
            self.velocity_depth.encode(),
            0, 0, 0, 0, 0, 0, 0,  // seven dummy bytes by design
        ]
    }
}

impl Checksum for Common {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        (total & 0x7F) as u8
    }
}

impl SystemExclusiveData for Common {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Common {
            channel: MidiChannel::new((data[0] + 1) as i32),
            volume: Level::new(data[1] as i32),
            velocity_depth: ModulationDepth::new((data[2] as i32) - 50),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size() -> usize {
        3 + 7 + 1 // include the seven dummy bytes and checksum
    }
}

pub const NOTE_DATA_SIZE: usize = 1 + 2 * SOURCE_DATA_SIZE;

/// Drum note.
#[derive(Copy, Clone)]
pub struct Note {
    pub submix: Submix,
    pub source1: Source,
    pub source2: Source,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            submix: Submix::A,
            source1: Default::default(),
            source2: Default::default(),
        }
    }
}

impl Note {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        // Get the bytes for S1 and S2 separately, then interleave them
        let mut source1_bytes = self.source1.to_bytes();
        let source2_bytes = self.source2.to_bytes();

        // Inject the output select into the first byte of S1
        source1_bytes[0].set_bit_range(4..7, self.submix as u8);

        for i in 0..source1_bytes.len() {
            buf.push(source1_bytes[i]);
            buf.push(source2_bytes[i]);
        }

        buf
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "submix = {}, source 1 = {}, source 2 = {}",
            self.submix, self.source1, self.source2
        )
    }
}

impl Checksum for Note {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        (total & 0x7F) as u8
    }
}

impl SystemExclusiveData for Note {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        // The bytes have S1 and S2 interleaved, so group them:
        let mut source1_bytes = Vec::<u8>::new();
        let mut source2_bytes = Vec::<u8>::new();
        let mut i = 0;
        while i < 10 {
            source1_bytes.push(data[i]);
            i += 1;
            source2_bytes.push(data[i]);
            i += 1;
        }

        // Get the submix from S1 byte 0:
        let submix = Submix::try_from(source1_bytes[0] >> 4).unwrap();

        // Then mask it away:
        source1_bytes[0] &= 0b00001111;

        Ok(Note {
            submix,
            source1: Source::parse(&source1_bytes)?,
            source2: Source::parse(&source2_bytes)?,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());

        buf
    }

    fn data_size() -> usize {
        1 // include checksum
        + 2 * SOURCE_DATA_SIZE
    }
}

pub const SOURCE_DATA_SIZE: usize = 5;

/// Drum source.
#[derive(Copy, Clone)]
pub struct Source {
    pub wave: Wave, // 1~256
    pub decay: Decay, // 1~100
    pub tune: ModulationDepth, // -50~+50
    pub level: Level, // 0~100
}

impl Default for Source {
    fn default() -> Self {
        Source {
            wave: Wave::new(),
            decay: Decay::new(1),
            tune: ModulationDepth::new(0),
            level: Level::new(100),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "wave = {} ({}), decay = {}, tune = {}, level = {}",
            self.wave.name(), self.wave.number,
            self.decay,
            self.tune,
            self.level
        )
    }
}

impl SystemExclusiveData for Source {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Source {
            wave: Wave::parse(&[data[0], data[1]])?,
            decay: parse_or_default::<Decay>(data[2]),
            tune: parse_or_default::<ModulationDepth>(data[3]),  // adjust to -50~+50
            level: parse_or_default::<Level>(data[4]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(self.wave.to_bytes());
        buf.push(self.decay.encode());
        buf.push(self.tune.encode());
        buf.push(self.level.encode());
        buf
    }

    fn data_size() -> usize { 5 }
}

#[cfg(test)]
mod tests {
    use super::{*};

    use crate::k4::{
        bank,
        sysex::Header,
        single::SinglePatch,
        multi::MultiPatch,
        drum::DrumPatch
    };

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_drum_patch_from_bytes() {
        let start: usize = dbg!(
            2 +
            Header::data_size() +
            bank::SINGLE_PATCH_COUNT * SinglePatch::data_size() +
            bank::MULTI_PATCH_COUNT * MultiPatch::data_size());
        let patch = DrumPatch::parse(&DATA[start..]);
        assert_eq!(patch.unwrap().common.volume.value(), 0x64);
    }

}
