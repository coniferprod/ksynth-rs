//! System Exclusive data definitions for K5000.
//!

use std::convert::TryFrom;
use std::fmt;
use num_enum::TryFromPrimitive;
use crate::{SystemExclusiveData, ParseError};
use crate::k5000::MIDIChannel;

/// Kawai K5000 System Exclusive functions.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Function {
    OneBlockDumpRequest = 0x00,
    AllBlockDumpRequest = 0x01,
    ParameterSend = 0x10,
    TrackControl = 0x11,
    OneBlockDump = 0x20,
    AllBlockDump = 0x21,
    ModeChange = 0x31,
    Remote = 0x32,
    WriteComplete = 0x40,
    WriteError = 0x41,
    WriteErrorByProtect = 0x42,
    WriteErrorByMemoryFull = 0x44,
    WriteErrorByNoExpandedMemory = 0x45,
}

/// K5000 System Exclusive message.
pub struct Message {
    pub channel: MIDIChannel,
    pub function: Function,
    pub function_data: Vec<u8>,
    pub subdata: Vec<u8>,
    pub patch_data: Vec<u8>,
}

impl SystemExclusiveData for Message {
    fn from_bytes(data: Vec<u8>) -> Result<Self, ParseError> {
        Ok(Message {
            channel: MIDIChannel::new(data[2].into()),
            function: Function::try_from(data[3]).unwrap(),
            function_data: Vec::<u8>::new(),  // TODO: fix this
            subdata: Vec::<u8>::new(),  // TODO: fix this
            patch_data: data[3..].to_vec(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(0x40); // Kawai manufacturer ID
        result.push(self.channel.value() as u8);

        result.push(self.function as u8);
        result.extend(&self.function_data);
        result.extend(&self.subdata);
        result.extend(&self.patch_data);

        result
    }
}


/// Cardinality of SysEx message (one patch or block of patches).
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Cardinality {
    One = 0x20,
    Block = 0x21,
}

impl From<Cardinality> for u8 {
    fn from(c: Cardinality) -> u8 {
        c as u8
    }
}

/// K5000 bank identifier.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum BankIdentifier {
    A = 0x00,
    B = 0x01,
    // there is no Bank C
    D = 0x02,  // only on K5000S/R
    E = 0x03,
    F = 0x04,
}

impl TryFrom<u8> for BankIdentifier {
    type Error = ();

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            x if x == BankIdentifier::A as u8 => Ok(BankIdentifier::A),
            x if x == BankIdentifier::B as u8 => Ok(BankIdentifier::B),
            x if x == BankIdentifier::D as u8 => Ok(BankIdentifier::D),
            x if x == BankIdentifier::E as u8 => Ok(BankIdentifier::E),
            x if x == BankIdentifier::F as u8 => Ok(BankIdentifier::F),
            _ => Err(()),
        }
    }
}

/// Patch kind.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum PatchKind {
    Single = 0x00,
    Multi = 0x20, // combi on K5000W
    DrumKit = 0x10,
    DrumInstrument = 0x11,
}

/// System Exclusive dump header.
#[derive(Debug, PartialEq)]
pub struct Header {
    pub channel: MIDIChannel,
    pub cardinality: Cardinality,
    pub bank_identifier: Option<BankIdentifier>,
    pub kind: PatchKind,
    pub sub_bytes: Vec<u8>,
}

impl Header {
    /// Identifies a dump header from a byte vector.
    ///
    /// Returns `Some(Header)` if the header could be parsed,
    /// `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `buf` - a byte vector with the header data
    pub fn identify_vec(buf: &Vec<u8>) -> Option<Header> {
        let channel = MIDIChannel::from(buf[0]);  // will be converted to 1...16
        match &buf[1..] {
            // One ADD Bank A (see 3.1.1b)
            [0x20, 0x00, 0x0A, 0x00, 0x00, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::A),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
            },

            // One PCM Bank B (see 3.1.1d)
            [0x20, 0x00, 0x0A, 0x00, 0x01, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::B),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
            },

            // One ADD Bank D (see 3.1.1k)
            [0x20, 0x00, 0x0A, 0x00, 0x02, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::D),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
            },

            // One Exp Bank E (see 3.1.1m)
            [0x20, 0x00, 0x0A, 0x00, 0x03, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::E),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1],
                })
            },

            // One Exp Bank F (see 3.1.1o)
            [0x20, 0x00, 0x0A, 0x00, 0x04, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::F),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1],
                })
            },

            // One Multi/Combi (see 3.1.1i)
            [0x20, 0x00, 0x0A, 0x20, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::Multi,
                    sub_bytes: vec![*sub1],
                })
            },

            // Block ADD Bank A (see 3.1.1a)
            [0x21, 0x00, 0x0A, 0x00, 0x00, tone_map @ ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::A),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block PCM Bank B -- all PCM data, no tone map
            [0x21, 0x00, 0x0A, 0x00, 0x01, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::B),
                    kind: PatchKind::Single,
                    sub_bytes: vec![],
                })
            },

            // Block ADD Bank D (see 3.1.1j)
            [0x21, 0x00, 0x0A, 0x00, 0x02, tone_map @ ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::D),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Exp Bank E (see 3.1.1l)
            [0x21, 0x00, 0x0A, 0x00, 0x03, tone_map @ ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::E),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Exp Bank F (see 3.1.1n)
            [0x21, 0x00, 0x0A, 0x00, 0x04, tone_map @ ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::F),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Multi/Combi (see 3.1.1h)
            [0x21, 0x00, 0x0A, 0x20, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: None,
                    kind: PatchKind::Multi,
                    sub_bytes: vec![],
                })
            },

            // One drum kit (see 3.1.1e)
            [0x20, 0x00, 0x0A, 0x10, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::DrumKit,
                    sub_bytes: vec![],
                })
            },

            // One drum instrument (see 3.1.1g)
            [0x20, 0x00, 0x0A, 0x11, sub1, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::DrumInstrument,
                    sub_bytes: vec![*sub1],
                })
            },

            // Block drum instrument (see 3.1.1f)
            [0x21, 0x00, 0x0A, 0x11, ..] => {
                Some(Header {
                    channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: None,
                    kind: PatchKind::DrumInstrument,
                    sub_bytes: vec![],
                })
            },

            // All others (must have this arm with slice patterns)
            _ => { None }
        }

    }

    // Returns the size of this dump command in bytes
    pub fn size(&self) -> usize {
        1 + // channel     ("3rd" in K5000 MIDI spec)
        1 + // cardinality ("4th" in K5000 MIDI spec)
        1 + // 0x00 ("5th")
        1 + // 0x0A ("6th")
        1 + // kind ("7th")
        self.sub_bytes.len()  // 0 to max 19 (if block tone map present)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(bank) = &self.bank_identifier {
            write!(f, "{:?} {:?} for Bank {:?}, {}",
                self.cardinality,
                self.kind,
                bank,
                if self.cardinality == Cardinality::One {
                    self.sub_bytes[0].to_string()
                }
                else {
                    "N/A".to_string()
                }
            )
        }
        else {
            write!(f, "{:?} {:?}, {}",
                self.cardinality,
                self.kind,
                if self.cardinality == Cardinality::One {
                    if self.kind != PatchKind::DrumKit {
                        self.sub_bytes[0].to_string()
                    }
                    else {
                        "N/A".to_string()
                    }
                }
                else {
                    "N/A".to_string()
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_one_add_bank_a() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x00 ]; // One ADD Bank A
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: Some(BankIdentifier::A),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_one_add_bank_d() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x00, 0x02, 0x00 ]; // One ADD Bank D
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: Some(BankIdentifier::D),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_one_exp_bank_e() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x00, 0x03, 0x00 ]; // One Exp Bank E
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: Some(BankIdentifier::E),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_one_exp_bank_f() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x00, 0x04, 0x00 ]; // One Exp Bank F
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: Some(BankIdentifier::F),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_one_multi() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x20, 0x00 ]; // One Multi
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: None,
                kind: PatchKind::Multi,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_block_add_bank_a() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x00, 0x00,
            /* tone map of 19 bytes follows */
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];  // Block ADD Bank A

        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: Some(BankIdentifier::A),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00; 19]
            }
        );
    }

    #[test]
    fn test_block_add_bank_d() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x00, 0x02,
                    /* tone map of 19 bytes follows */
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];  // Block ADD Bank D
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: Some(BankIdentifier::D),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00; 19]
            }
        );
    }

    #[test]
    fn test_block_exp_bank_e() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x00, 0x03,
            /* tone map of 19 bytes follows */
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];  // Block Exp Bank E
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: Some(BankIdentifier::E),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00; 19]
            }
        );
    }

    #[test]
    fn test_block_exp_bank_f() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x00, 0x04,
            /* tone map of 19 bytes follows */
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];  // Block Exp Bank F
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: Some(BankIdentifier::F),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00; 19]
            }
        );
    }

    #[test]
    fn test_block_multi() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x20 ]; // Block Multi
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: None,
                kind: PatchKind::Multi,
                sub_bytes: vec![]
            }
        );
    }

    #[test]
    fn test_one_pcm_bank_b() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x00, 0x01, 0x00 ]; // One PCM Bank B
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: Some(BankIdentifier::B),
                kind: PatchKind::Single,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_block_pcm_bank_b() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x00, 0x01 ]; // Block PCM Bank B
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: Some(BankIdentifier::B),
                kind: PatchKind::Single,
                sub_bytes: vec![]
            }
        );
    }

    #[test]
    fn test_one_drum_kit() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x10 ]; // One Drum Kit
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: None,
                kind: PatchKind::DrumKit,
                sub_bytes: vec![]
            }
        );
    }

    #[test]
    fn test_one_drum_instrument() {
        let cmd: Vec<u8> = vec![ 0x00, 0x20, 0x00, 0x0A, 0x11, 0x00 ]; // One Drum Instrument
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::One,
                bank_identifier: None,
                kind: PatchKind::DrumInstrument,
                sub_bytes: vec![0x00]
            }
        );
    }

    #[test]
    fn test_block_drum_instrument() {
        let cmd: Vec<u8> = vec![ 0x00, 0x21, 0x00, 0x0A, 0x11 ]; // Block Drum Instrument
        assert_eq!(
            Header::identify_vec(&cmd).unwrap(),
            Header {
                channel: MIDIChannel::new(1),
                cardinality: Cardinality::Block,
                bank_identifier: None,
                kind: PatchKind::DrumInstrument,
                sub_bytes: vec![]
            }
        );
    }
}
