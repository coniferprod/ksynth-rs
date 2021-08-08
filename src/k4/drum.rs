//! Data model for drum patches.
//!

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use bit::BitIndex;
use crate::{SystemExclusiveData, Checksum};
use crate::k4::DRUM_NOTE_COUNT;
use crate::k4::wave::Wave;
use crate::k4::effect::Submix;

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

impl SystemExclusiveData for DrumPatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        let common = Common::from_bytes(data[0..].to_vec());
        let mut offset = common.data_size();
        let mut notes = [Default::default(); DRUM_NOTE_COUNT];

        for i in 0..DRUM_NOTE_COUNT {
            eprintln!("Parsing drum note {}, offset = {}", i, offset);

            let note = Note::from_bytes(data[offset..].to_vec());
            notes[i] = note;
            offset += note.data_size();
        }

        DrumPatch {
            common,
            notes,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        // Note: the drum patch doesn't have an overall checksum
        buf
    }

    fn data_size(&self) -> usize {
        self.common.data_size()
            + self.notes[0].data_size() * DRUM_NOTE_COUNT
    }
}

/// Drum common data.
pub struct Common {
    pub channel: u8,  // MIDI channel, here 1...16, stored in SysEx as 0...15
    pub volume: u16, // 0~100
    pub velocity_depth: u16,  // 0~100
}

impl Default for Common {
    fn default() -> Self {
        Common {
            channel: 10,
            volume: 100,
            velocity_depth: 0,
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
            self.channel, self.volume, self.velocity_depth
        )
    }
}

impl Common {
    fn collect_data(&self) -> Vec<u8> {
        vec![
            self.channel - 1,
            self.volume as u8,
            self.velocity_depth as u8,
            0, 0, 0, 0, 0, 0, 0,
        ]
    }
}

impl Checksum for Common {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        ((total & 0x7F) as u8).try_into().unwrap()
    }
}

impl SystemExclusiveData for Common {
    fn from_bytes(data: Vec<u8>) -> Self {
        Common {
            channel: data[0] + 1,
            volume: data[1] as u16,
            velocity_depth: data[2] as u16,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size(&self) -> usize {
        3 + 7 + 1 // include the seven dummy bytes and checksum
    }
}

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

impl Checksum for Note {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        ((total & 0x7F) as u8).try_into().unwrap()
    }
}

impl SystemExclusiveData for Note {
    fn from_bytes(data: Vec<u8>) -> Self {
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

        Note {
            submix,
            source1: Source::from_bytes(source1_bytes),
            source2: Source::from_bytes(source2_bytes),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());

        buf
    }

    fn data_size(&self) -> usize {
        1 // include checksum
            + self.source1.data_size()
            + self.source2.data_size()
    }
}

/// Drum source.
#[derive(Copy, Clone)]
pub struct Source {
    pub wave: Wave, // 1~256
    pub decay: u16, // 1~100
    pub tune: i16, // -50~+50
    pub level: u16, // 0~100
}

impl Default for Source {
    fn default() -> Self {
        Source {
            wave: Wave::new(),
            decay: 1,
            tune: 0,
            level: 100,
        }
    }
}

impl SystemExclusiveData for Source {
    fn from_bytes(data: Vec<u8>) -> Self {
        Source {
            wave: Wave::from_bytes(vec![data[0], data[1]]),
            decay: data[2] as u16,
            tune: (data[3] as i16) - 50,  // adjust to -50~+50
            level: data[4] as u16,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(self.wave.to_bytes());
        buf.push(self.decay as u8);
        buf.push((self.tune + 50) as u8);
        buf.push(self.level as u8);
        buf
    }

    fn data_size(&self) -> usize {
        5
    }
}
