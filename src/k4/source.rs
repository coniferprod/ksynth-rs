use std::fmt;
use bit::BitIndex;
use crate::k4::wave::Wave;

#[derive(Copy, Clone)]
pub struct Source {
    pub delay: u8,
    pub wave: Wave,
    pub ks_curve: u8,
    pub coarse: i8,
    pub key_track: KeyTrack,
    pub fine: i8,
    pub press_freq: bool,
    pub vibrato: bool,
    pub velocity_curve: u8,  // 1~8 (in SysEx 0~7)
}

impl Source {
    pub fn new() -> Source {
        Source {
            delay: 0,
            wave: Default::default(),
            ks_curve: 1,
            coarse: 0,
            key_track: KeyTrack::On,
            fine: 0,
            press_freq: true,
            vibrato: true,
            velocity_curve: 1,
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
            self.delay, self.wave, self.ks_curve, self.coarse, self.fine, self.key_track, self.press_freq, self.vibrato, self.velocity_curve
        )
    }
}

impl crate::SystemExclusiveData for Source {
    fn from_bytes(data: Vec<u8>) -> Self {
        let mut offset: usize = 0;

        let mut b: u8;
        b = data[offset];
        offset += 1;
        let delay = b & 0x7f;

        b = data[offset];
        offset += 1;
        let wave_high = b & 0x01;
        let ks_curve = ((b >> 4) & 0x07) + 1; // 0...7 to 1...8
        let wave_low = data[offset] & 0x7f;
        offset += 1;

        let wave = Wave::from_bytes(vec![wave_high, wave_low]);

        b = data[offset];
        offset += 1;

        // Assuming that the low six bits are the coarse value,
        // and b6 is the key tracking bit (b7 is zero).
        let is_key_track = b.bit(6);

        let coarse = ((b & 0x3f) as i8) - 24;  // 00 ~ 48 to ±24

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
        let fine = ((b & 0x7f) as i8) - 50;

        b = data[offset];
        let press_freq = b.bit(0);
        let vibrato = b.bit(1);
        let velocity_curve = ((b >> 2) & 0x07) + 1;  // 0...7 to 1...8

        Source {
            delay,
            wave,
            ks_curve,
            coarse,
            key_track,
            fine,
            press_freq,
            vibrato,
            velocity_curve,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.delay);

        let mut s34 = self.ks_curve << 4;
        let wave_bytes = self.wave.to_bytes();
        if wave_bytes[0] == 1 {
            s34.set_bit(0, true);
        }
        buf.push(s34);
        buf.push(wave_bytes[1]);

        let mut s42 = (self.coarse + 24) as u8;  // bring into 0~48
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

        buf.push((self.fine + 50) as u8);  // bring into 0~100

        let mut s54 = (self.velocity_curve - 1) << 2;
        if self.vibrato {
            s54.set_bit(0, true);
        }
        buf.push(s54);

        buf
    }

    fn data_size(&self) -> usize { 7 }
}

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
