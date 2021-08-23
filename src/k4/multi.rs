use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;
use bit::BitIndex;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;
use crate::Checksum;
use crate::k4;

const SECTION_COUNT: usize = 8;  // number of sections in a multi

#[derive(Clone)]
pub struct MultiPatch {
    pub name: String,
    pub volume: u8,
    pub effect: u8,
    pub sections: Vec<Section>,
}

impl MultiPatch {
    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.extend(self.name.as_bytes());
        buf.push(self.volume);
        buf.push(self.effect - 1);  // adjust 1~32 to 0~31

        for i in 0..8 {
            buf.extend(self.sections[i].to_bytes());
        }

        buf
    }
}

impl Default for MultiPatch {
    fn default() -> Self {
        MultiPatch {
            name: "NewMulti  ".to_string(),
            volume: 100,
            effect: 1,
            sections: vec![Default::default(); SECTION_COUNT],
        }
    }
}

impl fmt::Display for MultiPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} volume={} effect={}",
            self.name, self.volume, self.effect)

            // TODO: Write the sections too
    }
}

impl SystemExclusiveData for MultiPatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;

        // name = M0 ... M9
        end = start + crate::k4::NAME_LENGTH;
        let name = String::from_utf8(data[start..end].to_vec()).unwrap();
        offset += crate::k4::NAME_LENGTH + 2;  // skip over name, volume and effect to sections

        let mut sections = Vec::<Section>::new();
        for i in 0..SECTION_COUNT {
            sections.push(Section::from_bytes(data[offset .. offset + 8].to_vec()));
            offset += 8;
        }

        MultiPatch {
            name: name,
            volume: data[10],
            effect: data[11] + 1,  // use 1...32 for effect patch
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
    pub single_number: u8,
    pub zone: Zone,
    pub velocity_switch: VelocitySwitch,
    pub receive_channel: u8,
    pub is_muted: bool,
    pub out_select: u8,
    pub play_mode: PlayMode,
    pub level: u8,
    pub transpose: i8,  // +-24 (in SysEx 0~48)
    pub tune: i8,  // +-50 (in SysEx 0~100)
}

impl Section {
    pub fn new() -> Section {
        Section {
            single_number: 0,
            zone: Zone { low_key: 0, high_key: 127 },
            velocity_switch: VelocitySwitch::All,
            receive_channel: 1,  // use 1...16 for MIDI channel here
            is_muted: false,
            out_select: 0,
            play_mode: PlayMode::Keyboard,
            level: 100,
            transpose: 0,
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
            single_number: data[0],
            zone: Zone { low_key: data[1], high_key: data[2] },
            velocity_switch: VelocitySwitch::try_from((data[3] >> 4) & 0b0000_0011).unwrap(),
            receive_channel: (data[3] & 0b0000_1111) + 1,  // adjust MIDI channel to 1...16
            is_muted: if data[3] >> 6 == 1 { true } else { false },
            out_select: data[4] & 0b0000_0111,
            play_mode: PlayMode::try_from((data[4] >> 3) & 0b0000_0011).unwrap(),
            level: data[5],
            transpose: (data[6] as i8) - 24,
            tune: (data[7] as i8) - 50,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.single_number);
        buf.push(self.zone.low_key);
        buf.push(self.zone.high_key);

        let mut m15 = (self.receive_channel - 1) | ((self.velocity_switch as u8) << 4);
        m15.set_bit(6, if self.is_muted { true } else { false });
        buf.push(m15);

        let m16 = self.out_select | ((self.play_mode as u8) << 3);
        buf.push(m16);

        buf.push(self.level);
        buf.push((self.transpose + 24) as u8);
        buf.push((self.tune + 50) as u8);

        buf
    }

    fn data_size(&self) -> usize { 8 }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Zone {
    pub low_key: u8,
    pub high_key: u8,
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ... {}",
            k4::get_note_name(self.low_key),
            k4::get_note_name(self.high_key))
    }
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

    #[test]
    fn test_multi_patch_from_bytes() {
        let data: [u8; 77] = include!("a401multi1.in");
        let patch = MultiPatch::from_bytes(data.to_vec());
        assert_eq!(patch.name, "Fatt!Anna5");
        assert_eq!(patch.volume, 0x50);
    }
}
