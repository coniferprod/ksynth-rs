use num_enum::TryFromPrimitive;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Submix {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Submix {
    pub fn name(&self) -> String {
        match self {
            Submix::A => "A".to_string(),
            Submix::B => "B".to_string(),
            Submix::C => "C".to_string(),
            Submix::D => "D".to_string(),
            Submix::E => "E".to_string(),
            Submix::F => "F".to_string(),
            Submix::G => "G".to_string(),
            Submix::H => "H".to_string(),
        }
    }
}

impl fmt::Display for Submix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_submix_name() {
        let submix = Submix::A;
        assert_eq!(submix.name(), "A");
    }
}
