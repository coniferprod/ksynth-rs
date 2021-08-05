use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;

//
// LFO
//

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
enum Waveform {
    Triangle,
    Square,
    Sawtooth,
    Sine,
    Random,
}

impl Default for Waveform {
    fn default() -> Self { Waveform::Triangle }
}

pub struct LfoControl {
    depth: u8,
    key_scaling: i8,
}

impl Default for LfoControl {
    fn default() -> Self {
        LfoControl {
            depth: 0,
            key_scaling: 0,
        }
    }
}

impl SystemExclusiveData for LfoControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        LfoControl {
            depth: data[0],
            key_scaling: (data[1] - 64) as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.depth, (self.key_scaling + 64) as u8]
    }
}

pub struct Lfo {
    waveform: Waveform,
    speed: u8,
    fade_in_time: u8,
    fade_in_to_speed: u8,
    delay_onset: u8,
    vibrato: LfoControl,
    growl: LfoControl,
    tremolo: LfoControl,
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo {
            waveform: Default::default(),
            speed: 0,
            fade_in_time: 0,
            fade_in_to_speed: 0,
            delay_onset: 0,
            vibrato: Default::default(),
            growl: Default::default(),
            tremolo: Default::default(),
        }
    }
}

impl SystemExclusiveData for Lfo {
    fn from_bytes(data: Vec<u8>) -> Self {
        Lfo {
            waveform: Waveform::try_from(data[0]).unwrap(),
            speed: data[1],
            fade_in_time: data[2],
            fade_in_to_speed: data[3],
            delay_onset: data[4],
            vibrato: LfoControl {
                depth: data[5],
                key_scaling: data[6] as i8,
            },
            growl: LfoControl {
                depth: data[7],
                key_scaling: data[8] as i8,
            },
            tremolo: LfoControl {
                depth: data[9],
                key_scaling: data[10] as i8,
            },
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(vec![self.waveform as u8, self.speed, self.delay_onset, self.fade_in_time, self.fade_in_to_speed]);
        result.extend(self.vibrato.to_bytes());
        result.extend(self.growl.to_bytes());
        result.extend(self.tremolo.to_bytes());

        result
    }
}
