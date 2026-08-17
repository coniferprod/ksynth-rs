//! Data model for source in a single patch.
//!

use std::fmt;

use bit::BitIndex;
use syxpack::{
    Ranged, 
    SystemExclusiveData, 
    ParseError,
    parse_or_default,
    Encoding,
};

use crate::k4::{Level, Curve, Coarse, Fine};
use crate::k4::wave::Wave;

/// Source in a single patch.
#[derive(Copy, Clone)]
pub struct Source {
    pub delay: Level,
    pub wave: Wave,
    pub ks_curve: Curve,
    pub coarse: Coarse,
    pub key_track: KeyTrack,
    pub fine: Fine,
    pub press_freq: bool,
    pub vibrato: bool,
    pub velocity_curve: Curve,  // 1~8 (in SysEx 0~7)
}

impl Source {
    pub fn new() -> Source {
        Source {
            delay: Level::new(0),
            wave: Default::default(),
            ks_curve: Curve::new(1),
            coarse: Coarse::new(0),
            key_track: KeyTrack::On,
            fine: Fine::new(0),
            press_freq: true,
            vibrato: true,
            velocity_curve: Curve::new(1),
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source::new()
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "delay = {}, wave = {}, KS curve = {}, coarse = {}, fine = {}, key track = {}, prs>freq = {}, vib>a.bend = {}, vel.curve = {}",
            self.delay,
            self.wave,
            self.ks_curve,
            self.coarse,
            self.fine,
            self.key_track,
            self.press_freq,
            self.vibrato,
            self.velocity_curve
        )
    }
}

impl SystemExclusiveData for Source {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        let mut offset: usize = 0;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let delay = b & 0x7f;

        b = data[offset];
        offset += 1;
        let wave_high = b & 0x01;
        let ks_curve = (b >> 4) & 0x07;
        let wave_low = data[offset] & 0x7f;
        offset += 1;

        let wave = Wave::parse(&[wave_high, wave_low]);

        b = data[offset];
        offset += 1;

        // Assuming that the low six bits are the coarse value,
        // and b6 is the key tracking bit (b7 is zero).
        let is_key_track = b.bit(6);

        let coarse = b & 0x3f;

        b = data[offset];
        offset += 1;
        let fixed_key = b & 0x7f;

        let key_track = if is_key_track {
            KeyTrack::On
        }
        else {
            KeyTrack::FixedKey(fixed_key)
        };

        b = data[offset];
        offset += 1;
        let fine = b & 0x7f;

        b = data[offset];
        let press_freq = b.bit(0);
        let vibrato = b.bit(1);
        let velocity_curve = (b >> 2) & 0x07;

        Ok(Source {
            delay: parse_or_default::<Level>(delay),
            wave: wave?,
            ks_curve: parse_or_default::<Curve>(ks_curve),
            coarse: parse_or_default::<Coarse>(coarse),
            key_track,
            fine: parse_or_default::<Fine>(fine),
            press_freq,
            vibrato,
            velocity_curve: parse_or_default::<Curve>(velocity_curve),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.delay.encode());

        let mut s34 = self.ks_curve.encode() << 4;
        let wave_bytes = self.wave.to_bytes();
        if wave_bytes[0] == 1 {
            s34.set_bit(0, true);
        }
        buf.push(s34);
        buf.push(wave_bytes[1]);

        let mut s42 = self.coarse.encode();
        let mut key: u8 = 0;
        match self.key_track {
            KeyTrack::On => {
                s42.set_bit(6, true);
            },
            KeyTrack::FixedKey(k) => {
                key = k;
            }
        };
        buf.push(s42);
        buf.push(key);

        buf.push(self.fine.encode());

        let mut s54 = self.velocity_curve.encode() << 2;
        if self.vibrato {
            s54.set_bit(0, true);
        }
        buf.push(s54);

        buf
    }

    fn data_size() -> usize { 7 }
}

/// Key tracking setting.
#[derive(Copy, Clone)]
pub enum KeyTrack {
    On,
    FixedKey(u8),
}

impl fmt::Display for KeyTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyTrack::On => "ON".to_string(),
                KeyTrack::FixedKey(k) => format!("fixed {}", k),
            }
        )
    }
}
