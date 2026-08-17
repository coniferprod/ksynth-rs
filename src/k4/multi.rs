//! Data model for multi patches.
//!

use std::fmt;
use std::convert::TryFrom;

use bit::BitIndex;
use num_enum::TryFromPrimitive;

use syxpack::{
    Ranged,
    SystemExclusiveData,
    ParseError,
    MidiChannel,
    Encoding,
    parse_or_default,
};

use crate::{
    Checksum, 
    MidiNote
};
use crate::k4::{
    Level,
    PatchNumber,
    EffectNumber,
    Transpose,
};

pub const DATA_SIZE: usize = 77;

/// Number of sections in a multi patch.
pub const SECTION_COUNT: usize = 8;

/// Multi patch.
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
        buf.push(self.volume.encode());
        buf.push(self.effect.encode());  // adjust 1~32 to 0~31

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
            volume: Level::new(100),
            effect: EffectNumber::new(1),
            sections: [Default::default(); SECTION_COUNT],
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
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset: usize = 0;
        let start: usize = 0;

        // name = M0 ... M9
        let end = start + crate::k4::NAME_LENGTH;

        let name = String::from_utf8(data[start..end].to_vec()).expect("Found invalid UTF-8");
        let name = str::replace(&name, char::from(0), " ").to_string();

        offset += crate::k4::NAME_LENGTH + 2;  // skip over name, volume and effect to sections

        let mut sections: [Section; SECTION_COUNT] = [Default::default(); SECTION_COUNT];
        for i in 0..SECTION_COUNT {
            sections[i] = Section::parse(&data[offset .. offset + 8])?;
            offset += 8;
        }

        Ok(MultiPatch {
            name,
            volume: parse_or_default::<Level>(data[10]),
            effect: parse_or_default::<EffectNumber>(data[11]),
            sections,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size() -> usize { DATA_SIZE }
}

impl Checksum for MultiPatch {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        (total & 0x7F) as u8
    }
}

/// Section of a multi patch.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Section {
    pub single_number: PatchNumber,
    pub zone: Zone,
    pub velocity_switch: VelocitySwitch,
    pub receive_channel: MidiChannel,
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
            single_number: PatchNumber::new(0),
            zone: Zone {
                low_key: MidiNote::new(0),
                high_key: MidiNote::new(127),
            },
            velocity_switch: VelocitySwitch::All,
            receive_channel: MidiChannel::new(1),
            is_muted: false,
            out_select: 0,
            play_mode: PlayMode::Keyboard,
            level: Level::new(100),
            transpose: Transpose::new(0),
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
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Section {
            single_number: PatchNumber::new(data[0].into()),
            zone: Zone::parse(&[data[1], data[2]])?,
            velocity_switch: VelocitySwitch::try_from((data[3] >> 4) & 0b0000_0011).unwrap(),
            receive_channel: parse_or_default::<MidiChannel>(data[3] & 0b0000_1111),
            is_muted: data[3] >> 6 == 1,
            out_select: data[4] & 0b0000_0111,
            play_mode: PlayMode::try_from((data[4] >> 3) & 0b0000_0011).unwrap(),
            level: parse_or_default::<Level>(data[5]),
            transpose: parse_or_default::<Transpose>(data[6]),
            tune: (data[7] as i8) - 50,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.single_number.value().try_into().unwrap());
        buf.push(self.zone.low_key.encode());
        buf.push(self.zone.high_key.encode());

        let mut m15 = (self.receive_channel.encode()) | ((self.velocity_switch as u8) << 4);
        m15.set_bit(6, self.is_muted);
        buf.push(m15);

        let m16 = self.out_select | ((self.play_mode as u8) << 3);
        buf.push(m16);

        buf.push(self.level.encode());
        buf.push(self.transpose.encode());
        buf.push((self.tune + 50) as u8);

        buf
    }

    fn data_size() -> usize { 8 }
}

/// Keyboard zone.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Zone {
    pub low_key: MidiNote,
    pub high_key: MidiNote,
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ... {}",
            self.low_key.name(),
            self.high_key.name())
    }
}

impl SystemExclusiveData for Zone {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(
            Zone {
                low_key: parse_or_default::<MidiNote>(data[0]),
                high_key: parse_or_default::<MidiNote>(data[1]),
            }
        )
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.low_key.encode(),
            self.high_key.encode(),
        ]
    }

    fn data_size() -> usize { 2 }
}

/// Velocity switch setting.
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

/// Play mode setting.
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

    use syxpack::Ranged;

    use crate::k4::{
        bank,
        sysex::Header,
        single::SinglePatch,
        multi::MultiPatch,
    };

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_multi_patch_from_bytes() {
        let start: usize = dbg!(
            2 +
            Header::data_size() +
            bank::SINGLE_PATCH_COUNT * SinglePatch::data_size());
        let patch = MultiPatch::parse(&DATA[start..]);
        assert_eq!(patch.as_ref().unwrap().name, "Fatt!Anna5");
        assert_eq!(patch.as_ref().unwrap().volume.value(), 0x50);
    }
}
