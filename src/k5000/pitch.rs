use crate::SystemExclusiveData;

//
// PitchEnvelope
//

pub struct Envelope {
    start: i8,
    attack_time: u8,
    attack_level: i8,
    decay_time: u8,
    time_vel_sens: i8,
    level_vel_sens: i8,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            start: 0,
            attack_time: 0,
            attack_level: 0,
            decay_time: 0,
            time_vel_sens: 0,
            level_vel_sens: 0,
        }
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            start: (data[0] - 64) as i8,
            attack_time: data[1],
            attack_level: (data[2] - 64) as i8,
            decay_time: data[3],
            time_vel_sens: (data[4] - 64) as i8,
            level_vel_sens: (data[5] - 64) as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let bs = vec![
            (self.start + 64) as u8,
            self.attack_time,
            (self.attack_level + 64) as u8,
            self.decay_time,
            (self.time_vel_sens + 64) as u8,
            (self.level_vel_sens + 64) as u8,
        ];
        result.extend(bs);
        result
    }
}
