use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;
use bit::BitIndex;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::Checksum;
use crate::k4;
use crate::k4::{Level, MIDIChannel, PatchNumber, EffectNumber, Transpose};

const SECTION_COUNT: usize = 8;  // number of sections in a multi

#[derive(Clone)]
pub struct MultiPatch {
    pub name: String,
    pub volume: Level,
    pub effect: EffectNumber,
    pub sections: [Section; SECTION_COUNT],
}

impl MultiPatch {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.extend(self.name.as_bytes());
        buf.push(self.volume.into_inner());
        buf.extend(self.effect.to_bytes());  // adjust 1~32 to 0~31

        for s in self.sections  {
            buf.extend(s.to_bytes());
        }

        buf
    }
}

impl Default for MultiPatch {
    fn default() -> Self {
        MultiPatch {
            name: "NewMulti  ".to_string(),
            volume: Level::new(100).unwrap(),
            effect: EffectNumber::new(1).unwrap(),
            sections: [Default::default(); SECTION_COUNT],
        }
    }
}

impl fmt::Display for MultiPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} volume={} effect={}",
            self.name, self.volume.into_inner(), self.effect.into_inner())

            // TODO: Write the sections too
    }
}

impl SystemExclusiveData for MultiPatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;
        let start: usize = 0;

        // name = M0 ... M9
        let end = start + crate::k4::NAME_LENGTH;

        let name = String::from_utf8(data[start..end].to_vec()).expect("Found invalid UTF-8");
        let name = str::replace(&name, char::from(0), " ").to_string();

        offset += crate::k4::NAME_LENGTH + 2;  // skip over name, volume and effect to sections

        let mut sections: [Section; SECTION_COUNT] = [Default::default(); SECTION_COUNT];
        for i in 0..SECTION_COUNT {
            sections[i] = Section::from_bytes(data[offset .. offset + 8].to_vec());
            offset += 8;
        }

        MultiPatch {
            name: name,
            volume: Level::new(data[10]).unwrap(),
            effect: EffectNumber::new(data[11]).unwrap(),
            sections: sections,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size(&self) -> usize { 77 }
}

impl Checksum for MultiPatch {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        ((total & 0x7F) as u8).try_into().unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Section {
    pub single_number: PatchNumber,
    pub zone: Zone,
    pub velocity_switch: VelocitySwitch,
    pub receive_channel: MIDIChannel,
    pub is_muted: bool,
    pub out_select: u8,
    pub play_mode: PlayMode,
    pub level: Level,
    pub transpose: Transpose,
    pub tune: i8,  // +-50 (in SysEx 0~100)
}

impl Section {
    pub fn new() -> Section {
        Section {
            single_number: PatchNumber::new(0).unwrap(),
            zone: Zone { low_key: Key { note: 0 }, high_key: Key { note: 127 } },
            velocity_switch: VelocitySwitch::All,
            receive_channel: MIDIChannel::new(1).unwrap(),  // use 1...16 for MIDI channel here
            is_muted: false,
            out_select: 0,
            play_mode: PlayMode::Keyboard,
            level: Level::new(100).unwrap(),
            transpose: Transpose::new(0).unwrap(),
            tune: 0,
        }
    }
}

impl Default for Section {
    fn default() -> Self {
        Section::new()
    }
}

impl SystemExclusiveData for Section {
    fn from_bytes(data: Vec<u8>) -> Self {
        Section {
            single_number: PatchNumber::new(data[0]).unwrap(),
            zone: Zone::from_bytes(vec![data[1], data[2]]),
            velocity_switch: VelocitySwitch::try_from((data[3] >> 4) & 0b0000_0011).unwrap(),
            receive_channel: MIDIChannel::from_bytes(vec![data[3] & 0b0000_1111]),  // adjust MIDI channel to 1...16
            is_muted: if data[3] >> 6 == 1 { true } else { false },
            out_select: data[4] & 0b0000_0111,
            play_mode: PlayMode::try_from((data[4] >> 3) & 0b0000_0011).unwrap(),
            level: Level::new(data[5]).unwrap(),
            transpose: Transpose::from_bytes(vec![data[6]]),
            tune: (data[7] as i8) - 50,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.single_number.into_inner());
        buf.push(self.zone.low_key.note);
        buf.push(self.zone.high_key.note);

        let mut m15 = (self.receive_channel.to_bytes()[0]) | ((self.velocity_switch as u8) << 4);
        m15.set_bit(6, if self.is_muted { true } else { false });
        buf.push(m15);

        let m16 = self.out_select | ((self.play_mode as u8) << 3);
        buf.push(m16);

        buf.push(self.level.into_inner());
        buf.push(self.transpose.to_bytes()[0]);
        buf.push((self.tune + 50) as u8);

        buf
    }

    fn data_size(&self) -> usize { 8 }
}

/// Key in a keyboard zone.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Key {
    /// MIDI note number for the key.
    pub note: u8,
}

impl Key {
    pub fn get_note_name(note_number: u8) -> String {
        let notes = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ];
        let octave = (note_number / 12) - 2;
        let name = notes[note_number as usize % 12];

        format!("{}{}", name, octave.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Zone {
    pub low_key: Key,
    pub high_key: Key,
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ... {}",
            k4::get_note_name(self.low_key.note),
            k4::get_note_name(self.high_key.note))
    }
}

impl SystemExclusiveData for Zone {
    fn from_bytes(data: Vec<u8>) -> Self {
        Zone { low_key: Key { note: data[0] }, high_key: Key { note: data[1] } }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.low_key.note, self.high_key.note]
    }

    fn data_size(&self) -> usize { 2 }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum VelocitySwitch {
    All,
    Soft,
    Loud,
}

impl fmt::Display for VelocitySwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            VelocitySwitch::All => "All",
            VelocitySwitch::Soft => "Soft",
            VelocitySwitch::Loud => "Loud",
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum PlayMode {
    Keyboard,
    Midi,
    Mix,
}

impl fmt::Display for PlayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            PlayMode::Keyboard => "Keyboard",
            PlayMode::Midi => "MIDI",
            PlayMode::Mix => "Mix",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{*};
    use crate::k4::bank::Bank;
    use syxpack::Message;

    #[test]
    fn test_multi_patch_from_bytes() {
        let data: [u8; 77] = include!("a401multi1.in");
        let patch = MultiPatch::from_bytes(data.to_vec());
        assert_eq!(patch.name, "Fatt!Anna5");
        assert_eq!(patch.volume.into_inner(), 0x50);
    }

/*
    #[test]
    fn test_a403_multi_a9() {
        let data: [u8; 15123] = include!("a403.in");
        match Message::new(&data.to_vec()) {
            Ok(message) => {
                match message {
                    Message::ManufacturerSpecific { manufacturer, payload } => {
                        let bank = Bank::from_bytes(payload);
                        let multi = &bank.multis[8];
                        assert_eq!(multi.name, "Solo Now! ");  // trailing NUL replaced by SPACE
                    },
                    _ => {
                        panic!("Not a manufacturer-specific message");
                    },
                }
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

 */
}
