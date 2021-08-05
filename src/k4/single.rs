use std::convert::TryInto;
use std::convert::TryFrom;
use bit::BitIndex;
use crate::{SystemExclusiveData, Checksum, every_nth_byte};
use num_enum::TryFromPrimitive;
use std::fmt;
use crate::k4::{NAME_LENGTH, get_effect_number};
use crate::k4::source::Source;
use crate::k4::lfo::*;
use crate::k4::amp::Amplifier;
use crate::k4::filter::Filter;
use crate::k4::effect::Submix;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum SourceMode {
    Normal,
    Twin,
    Double,
}

impl fmt::Display for SourceMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SourceMode::Normal => "Normal",
                SourceMode::Twin => "Twin",
                SourceMode::Double => "Double",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum PolyphonyMode {
    Poly1,
    Poly2,
    Solo1,
    Solo2,
}

impl fmt::Display for PolyphonyMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PolyphonyMode::Poly1 => "Poly 1",
                PolyphonyMode::Poly2 => "Poly 2",
                PolyphonyMode::Solo1 => "Solo 1",
                PolyphonyMode::Solo2 => "Solo 2",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum WheelAssign {
    Vibrato,
    Lfo,
    Dcf,
}

impl fmt::Display for WheelAssign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WheelAssign::Vibrato => "VIB",
                WheelAssign::Lfo => "LFO",
                WheelAssign::Dcf => "DCF",
            }
        )
    }
}

pub struct AutoBend {
    pub time: u8,
    pub depth: i8,
    pub key_scaling_time: i8,
    pub velocity_depth: i8,
}

impl Default for AutoBend {
    fn default() -> Self {
        AutoBend {
            time: 0,
            depth: 0,
            key_scaling_time: 0,
            velocity_depth: 0,
        }
    }
}

impl AutoBend {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for AutoBend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "time = {}, depth = {}, ks.time = {}, vel.depth = {}",
            self.time, self.depth, self.key_scaling_time, self.velocity_depth
        )
    }
}

impl SystemExclusiveData for AutoBend {
    fn from_bytes(data: Vec<u8>) -> Self {
        AutoBend {
            time: data[0] & 0x7f,
            depth: ((data[1] & 0x7f) as i8) - 50, // 0~100 to ±50
            key_scaling_time: ((data[2] & 0x7f) as i8) - 50, // 0~100 to ±50
            velocity_depth: ((data[3] & 0x7f) as i8) - 50, // 0~100 to ±50
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let b = vec![
            self.time,
            (self.depth + 50).try_into().unwrap(),
            (self.key_scaling_time + 50).try_into().unwrap(),
            (self.velocity_depth + 50).try_into().unwrap(),
        ];
        buf.extend(b);

        buf
    }

    fn data_size(&self) -> usize { 4 }
}

pub struct SinglePatch {
    pub name: String,
    pub volume: u8,  // 0~100
    pub effect: u8,  // 1~32 (in SysEx 0~31)
    pub submix: Submix,
    pub source_mode: SourceMode,
    pub polyphony_mode: PolyphonyMode,
    pub am12: bool,
    pub am34: bool,
    pub source_mutes: [bool; 4], // true if source is muted, false if not
    pub bender_range: u8,
    pub wheel_assign: WheelAssign,
    pub wheel_depth: i8,
    pub auto_bend: AutoBend,
    pub lfo: Lfo,
    pub vibrato: Vibrato,
    pub press_freq: i8,
    pub sources: [Source; 4],
    pub amplifiers: [Amplifier; 4],
    pub filter1: Filter,
    pub filter2: Filter,
}

impl SinglePatch {
    pub fn new() -> SinglePatch {
        SinglePatch {
            name: "NewSound  ".to_string(),
            volume: 100,
            effect: 1,
            submix: Submix::A,
            source_mode: SourceMode::Normal,
            polyphony_mode: PolyphonyMode::Poly1,
            am12: false,
            am34: false,
            source_mutes: [false, false, false, false],
            bender_range: 0,
            wheel_assign: WheelAssign::Dcf,
            wheel_depth: 0,
            auto_bend: Default::default(),
            lfo: Default::default(),
            vibrato: Default::default(),
            press_freq: 0,
            sources: [Default::default(), Default::default(), Default::default(), Default::default()],
            amplifiers: [Default::default(), Default::default(), Default::default(), Default::default()],
            filter1: Default::default(),
            filter2: Default::default(),
        }
    }

    // Ensure that the name is exactly 10 characters, contains only allowed characters,
    // and is padded from the right with spaces.
    pub fn set_name(&self, new_name: &str) {

    }

    fn collect_data(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.extend(self.name.as_bytes());
        buf.push(self.volume);
        buf.push(self.effect - 1);  // 1~32 to 0~31
        buf.push(self.submix as u8);

        let mut s13 = (self.polyphony_mode as u8) << 2;
        s13 |= self.source_mode as u8;
        if self.am12 {
            s13 |= 0b0010_0000;  // set bit #5
        }
        if self.am34 {
            s13 |= 0b0001_0000;  // set bit #4
        }
        buf.push(s13);

        let vibrato_bytes = self.vibrato.to_bytes();

        let mut s14 = vibrato_bytes[0] << 4;
        for i in 0..4 {
            s14.set_bit(i, if self.source_mutes[i] { true } else { false });
        }
        buf.push(s14);

        buf.push(((self.wheel_assign as u8) << 4) | self.bender_range);
        buf.push(vibrato_bytes[1]);  // s16, vib speed
        buf.push((self.wheel_depth + 50).try_into().unwrap());  // s17
        buf.extend(self.auto_bend.to_bytes());  // s18...s21
        buf.push(vibrato_bytes[2]);  // s22, vib pressure
        buf.push(vibrato_bytes[3]);  // s23, vib depth
        buf.extend(self.lfo.to_bytes());  // s24...s28
        buf.push((self.press_freq + 50).try_into().unwrap());  // s29

        // Source data
        let mut source_data: [Vec<u8>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for i in 0..4 {
            source_data[i] = self.sources[i].to_bytes();
        }

        for n in 0..7 {
            for i in 0..4 {
                buf.push(source_data[i][n]);
            }
        }

        // Amplifier data
        let mut amp_data: [Vec<u8>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for i in 0..4 {
            amp_data[i] = self.amplifiers[i].to_bytes();
        }

        for n in 0..11 {
            for i in 0..4 {
                buf.push(amp_data[i][n]);
            }
        }

        // Filter data
        let filter_data: [Vec<u8>; 2] = [
            self.filter1.to_bytes(),
            self.filter2.to_bytes(),
        ];
        for n in 0..14 {
            for i in 0..2 {
                buf.push(filter_data[i][n]);
            }
        }

        buf
    }

    fn source_mute_string(&self) -> String {
        let mut s = String::new();
        let chars: [char; 4] = ['1', '2', '3', '4'];
        for i in 0..4 {
            s.push(if self.source_mutes[i] { '-' } else { chars[i] });
        }
        s
    }
}

impl Default for SinglePatch {
    fn default() -> Self {
        SinglePatch::new()
    }
}

impl fmt::Display for SinglePatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut source_s = String::new();
        source_s.push_str("SOURCES: ");
        source_s.push_str(&self.source_mute_string());
        for i in 0..4 {
            source_s.push_str(&format!("Source {}:\n{}\n", i + 1, self.sources[i]));
            source_s.push_str(&format!("DCA:\n{}\n", self.amplifiers[i]));
            source_s.push_str(&format!("DCF:\nFilter 1:{}\nFilter 2:\n{}\n", self.filter1, self.filter2));
        }

        let mut param_s = String::new();
        param_s.push_str(&format!("p.bend = {}, wheel assign = {}, wheel depth = {}, vibrato: {}, auto bend = {}, LFO = {}, press.freq = {}",
            self.bender_range, self.wheel_assign, self.wheel_depth, self.vibrato, self.auto_bend, self.lfo, self.press_freq
        ));

        writeln!(f,
            "{} volume={} effect={} submix={} source mode={} polyphony mode={} AM1>2={} AM3>4={}\n{}\n{}",
            self.name, self.volume, self.effect, self.submix, self.source_mode, self.polyphony_mode,
            if self.am12 { "ON"} else { "OFF" }, if self.am34 { "ON" } else { "OFF" }, source_s, param_s)
    }
}

impl SystemExclusiveData for SinglePatch {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;

        // name = s00 ... s09
        start = 0;
        end = start + NAME_LENGTH;
        let name = String::from_utf8(data[start..end].to_vec()).unwrap();

        offset += NAME_LENGTH;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let volume = b;

        // effect = s11 bits 0...4
        b = data[offset];
        offset += 1;
        let effect = get_effect_number(b);

        // output select = s12 bits 0...2
        b = data[offset];
        offset += 1;
        let output_name_index = b & 0b00000111;
        let submix = Submix::try_from(output_name_index).unwrap();

        // source mode = s13 bits 0...1
        b = data[offset];
        offset += 1;
        let source_mode = b & 0x03;
        let polyphony_mode = (b >> 2) & 0x03;
        let am12 = ((b >> 4) & 0x01) == 1;
        let am34 = ((b >> 5) & 0x01) == 1;

        b = data[offset];
        offset += 1;
        // the source mute bits are in s14:
        // S1 = b0, S2 = b1, S3 = b2, S4 = b3
        // The K4 MIDI spec says 0/mute, 1/not mute,
        // so we flip it to make this value actually mean muted.
        let mut source_mutes = [true, true, true, true];
        for i in 0..crate::k4::SOURCE_COUNT {
            if crate::get_bit_at(b as u32, i as u8) {
                source_mutes[i] = false;
            }
        }

        let mut vibrato_bytes = Vec::<u8>::new();
        vibrato_bytes.push(b);

        b = data[offset];
        offset += 1;

        // Pitch bend = s15 bits 0...3
        let bender_range = b & 0x0f;
        // Wheel assign = s15 bits 4...5
        let wheel_assign = (b >> 4) & 0x03;

        b = data[offset];
        offset += 1;

        // Vibrato speed = s16 bits 0...6
        vibrato_bytes.push(b);

        // Wheel depth = s17 bits 0...6
        b = data[offset];
        offset += 1;
        let wheel_depth = ((b & 0x7f) as i8) - 50;  // 0~100 to ±50

        start = offset;
        end = offset + 4;
        let auto_bend = AutoBend::from_bytes(data[start..end].to_vec());
        offset += 4;

        b = data[offset];
        offset += 1;
        vibrato_bytes.push(b);  // vib pressure

        b = data[offset];
        offset += 1;
        vibrato_bytes.push(b);  // vib depth

        let vibrato = Vibrato::from_bytes(vibrato_bytes);

        start = offset;
        end = start + 5;
        let lfo = Lfo::from_bytes(data[start..end].to_vec());
        offset += 5;

        b = data[offset];
        offset += 1;
        let press_freq = ((b & 0x7f) as i8) - 50; // 0~100 to ±50

        let total_source_data_size = 4 * 7;
        start = offset;
        end = start + total_source_data_size;
        let all_source_data = data[start..end].to_vec();

        let s1 = Source::from_bytes(every_nth_byte(&all_source_data, 4, 0));
        let s2 = Source::from_bytes(every_nth_byte(&all_source_data, 4, 1));
        let s3 = Source::from_bytes(every_nth_byte(&all_source_data, 4, 2));
        let s4 = Source::from_bytes(every_nth_byte(&all_source_data, 4, 3));

        offset += total_source_data_size;

        let total_amp_data_size = 4 * 11;
        start = offset;
        end = start + total_amp_data_size;
        let all_amp_data = data[start..end].to_vec();

        let a1 = Amplifier::from_bytes(every_nth_byte(&all_amp_data, 4, 0));
        let a2 = Amplifier::from_bytes(every_nth_byte(&all_amp_data, 4, 1));
        let a3 = Amplifier::from_bytes(every_nth_byte(&all_amp_data, 4, 2));
        let a4 = Amplifier::from_bytes(every_nth_byte(&all_amp_data, 4, 3));

        offset += total_amp_data_size;

        let total_filter_data_size = 2 * 14;
        start = offset;
        end = start + total_filter_data_size;
        let all_filter_data = data[start..end].to_vec();

        let f1 = Filter::from_bytes(every_nth_byte(&all_filter_data, 2, 0));
        let f2 = Filter::from_bytes(every_nth_byte(&all_filter_data, 2, 1));

        offset += total_filter_data_size;

        b = data[offset];
        // "Check sum value (s130) is the sum of the A5H and s0 ~ s129".
        let original_checksum = b; // store the checksum as we got it from SysEx

        SinglePatch {
            name: name,
            volume: volume,
            effect: effect,
            submix: submix,
            source_mode: SourceMode::try_from(source_mode).unwrap(),
            polyphony_mode: PolyphonyMode::try_from(polyphony_mode).unwrap(),
            am12: am12,
            am34: am34,
            source_mutes: source_mutes,
            bender_range: bender_range,
            wheel_assign: WheelAssign::try_from(wheel_assign).unwrap(),
            wheel_depth: wheel_depth,
            auto_bend: auto_bend,
            lfo: lfo,
            vibrato: vibrato,
            press_freq: press_freq,
            sources: [s1, s2, s3, s4],
            amplifiers: [a1, a2, a3, a4],
            filter1: f1,
            filter2: f2,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let data = self.collect_data();
        buf.extend(data);
        buf.push(self.checksum());
        buf
    }

    fn data_size(&self) -> usize { 131 }
}

impl Checksum for SinglePatch {
    fn checksum(&self) -> u8 {
        let data = self.collect_data();
        let mut total = data.iter().fold(0, |acc, x| acc + ((*x as u32) & 0xFF));
        total += 0xA5;
        ((total & 0x7F) as u8).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_single_patch_from_bytes() {
        let data: [u8; 131] = include!("melovox1.in");
        let single_patch = SinglePatch::from_bytes(data.to_vec());
        assert_eq!(single_patch.name, "Melo Vox 1");
        assert_eq!(single_patch.volume, 100);
    }

    /*
    #[test]
    fn test_single_patch_format() {
        let data: [u8; 131] = include!("melovox1.in");
        let single_patch = SinglePatch::from_bytes(data.to_vec());
        assert_eq!(
            format!("{}", single_patch),
            "Melo Vox 1 volume=100 effect=1 submix=G source mode=Normal polyphony mode=Poly 2 AM1>2=OFF AM3>4=OFF"
        )
    }
    */
}
