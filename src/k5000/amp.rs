use crate::SystemExclusiveData;

/// Amplifier envelope.
#[derive(Debug)]
pub struct Envelope {
    pub attack_time: u8,
    pub decay1_time: u8,
    pub decay1_level: u8,
    pub decay2_time: u8,
    pub decay2_level: u8,
    pub release_time: u8,
}

impl Envelope {
    pub fn new() -> Envelope {
        Envelope {
            attack_time: 0,
            decay1_time: 0,
            decay1_level: 0,
            decay2_time: 0,
            decay2_level: 0,
            release_time: 0,
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl SystemExclusiveData for Envelope {
    fn from_bytes(data: Vec<u8>) -> Self {
        Envelope {
            attack_time: data[0],
            decay1_time: data[1],
            decay1_level: data[2],
            decay2_time: data[3],
            decay2_level: data[4],
            release_time: data[5],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.attack_time, self.decay1_time, self.decay1_level, self.decay2_time, self.decay2_level, self.release_time]
    }
}

pub struct KeyScalingControl {
    pub level: i8,
    pub attack_time: i8,
    pub decay1_time: i8,
    pub release: i8,
}

impl Default for KeyScalingControl {
    fn default() -> Self {
        KeyScalingControl {
            level: 0,
            attack_time: 0,
            decay1_time: 0,
            release: 0,
        }
    }
}

impl SystemExclusiveData for KeyScalingControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        KeyScalingControl {
            level: data[0] as i8,
            attack_time: data[1] as i8,
            decay1_time: data[2] as i8,
            release: data[3] as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(self.level - 64) as u8, (self.attack_time - 64) as u8, (self.decay1_time - 64) as u8, (self.release - 64) as u8]
    }
}

pub struct VelocityControl {
    pub level: u8,
    pub attack_time: i8,
    pub decay1_time: i8,
    pub release: i8,
}

impl Default for VelocityControl {
    fn default() -> Self {
        VelocityControl {
            level: 0,
            attack_time: 0,
            decay1_time: 0,
            release: 0,
        }
    }
}

impl SystemExclusiveData for VelocityControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        VelocityControl {
            level: data[0],
            attack_time: data[1] as i8,
            decay1_time: data[2] as i8,
            release: data[3] as i8,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.level, (self.attack_time - 64) as u8, (self.decay1_time - 64) as u8, (self.release - 64) as u8]
    }
}

pub struct Modulation {
    pub ks_to_env: KeyScalingControl,
    pub vel_to_env: VelocityControl,
}

impl Default for Modulation {
    fn default() -> Self {
        Modulation {
            ks_to_env: Default::default(),
            vel_to_env: Default::default(),
        }
    }
}

impl SystemExclusiveData for Modulation {
    fn from_bytes(data: Vec<u8>) -> Self {
        Modulation {
            ks_to_env: KeyScalingControl::from_bytes(data[..4].to_vec()),
            vel_to_env: VelocityControl::from_bytes(data[4..8].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.ks_to_env.to_bytes());
        result.extend(self.vel_to_env.to_bytes());

        result
    }
}

/// Amplifier.
pub struct Amplifier {
    pub velocity_curve: u8,
    pub envelope: Envelope,
    pub modulation: Modulation,
}

impl Default for Amplifier {
    fn default() -> Self {
        Amplifier {
            velocity_curve: 1,
            envelope: Default::default(),
            modulation: Default::default(),
        }
    }
}

impl SystemExclusiveData for Amplifier {
    fn from_bytes(data: Vec<u8>) -> Self {
        Amplifier {
            velocity_curve: data[0] + 1,  // 0-11 to 1-12
            envelope: Envelope::from_bytes(data[1..7].to_vec()),
            modulation: Modulation::from_bytes(data[7..15].to_vec()),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(self.velocity_curve - 1);
        result.extend(self.envelope.to_bytes());
        result.extend(self.modulation.to_bytes());

        result
    }
}
