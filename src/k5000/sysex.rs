//! System Exclusive data definitions.
//!

use std::convert::TryFrom;
use std::fmt;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum SystemExclusiveFunction {
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

/// K5000 System Exclusive Message.
pub struct SystemExclusiveMessage {
    pub channel: u8,
    pub function: SystemExclusiveFunction,
    pub function_data: Vec<u8>,
    pub subdata: Vec<u8>,
    pub patch_data: Vec<u8>,
}

impl SystemExclusiveData for SystemExclusiveMessage {
    fn from_bytes(data: Vec<u8>) -> Self {
        SystemExclusiveMessage {
            channel: data[2],
            function: SystemExclusiveFunction::try_from(data[3]).unwrap(),
            function_data: Vec::<u8>::new(),  // TODO: fix this
            subdata: Vec::<u8>::new(),  // TODO: fix this
            patch_data: data[3..].to_vec(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.push(0x40); // Kawai manufacturer ID
        result.push(self.channel - 1);  // 1...16 to 0...15

        result.push(self.function as u8);
        result.extend(&self.function_data);
        result.extend(&self.subdata);
        result.extend(&self.patch_data);

        result
    }
}


#[derive(Debug, PartialEq)]
pub enum Cardinality {
    One = 0x20,
    Block = 0x21,
}

#[derive(Debug, PartialEq)]
pub enum BankIdentifier {
    A = 0x00,
    B = 0x01,
    // there is no Bank C
    D = 0x02,  // only on K5000S/R
    E = 0x03,
    F = 0x04,
}

impl TryFrom<u8> for BankIdentifier
{
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

#[derive(Debug, PartialEq)]
enum PatchKind {
    Single = 0x00,
    Multi = 0x20, // combi on K5000W
    DrumKit = 0x10,
    DrumInstrument = 0x11,    
}

#[derive(Debug, PartialEq)]
struct DumpCommand {
    channel: u8,
    cardinality: Cardinality,
    bank_identifier: Option<BankIdentifier>,
    kind: PatchKind,
    sub_bytes: Vec<u8>,
}

impl DumpCommand {
    fn identify_vec(buf: &Vec<u8>) -> Option<DumpCommand> {
        match buf.as_slice() {
            // One ADD Bank A (see 3.1.1b)
            [channel, 0x20, 0x00, 0x0A, 0x00, 0x00, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::A),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
                //println!("channel={} ONE kind={} bank={} sub1={}", channel, kind, bank, sub1);
            },

            // One PCM Bank B (see 3.1.1d)
            [channel, 0x20, 0x00, 0x0A, 0x00, 0x01, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::B),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
            },

            // One ADD Bank D (see 3.1.1k)
            [channel, 0x20, 0x00, 0x0A, 0x00, 0x02, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::D),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1]
                })
            },

            // One Exp Bank E (see 3.1.1m)
            [channel, 0x20, 0x00, 0x0A, 0x00, 0x03, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::E),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1],
                })
                //println!("channel={} ONE kind={} bank={} sub1={} sub2={}", channel, kind, bank, sub1, sub2);
            },

            // One Exp Bank F (see 3.1.1o)
            [channel, 0x20, 0x00, 0x0A, 0x00, 0x04, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: Some(BankIdentifier::F),
                    kind: PatchKind::Single,
                    sub_bytes: vec![*sub1],
                })
                //println!("channel={} ONE kind={} bank={} sub1={} sub2={}", channel, kind, bank, sub1, sub2);
            },

            // One Multi/Combi (see 3.1.1i)
            [channel, 0x20, 0x00, 0x0A, 0x20, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::Multi,
                    sub_bytes: vec![*sub1],
                })
                //println!("channel={} ONE MULTI sub1={}", channel, sub1);
            },

            // Block ADD Bank A (see 3.1.1a)
            [channel, 0x21, 0x00, 0x0A, 0x00, 0x00, tone_map @ ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::A),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block PCM Bank B -- all PCM data, no tone map
            [channel, 0x21, 0x00, 0x0A, 0x00, 0x01, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::B),
                    kind: PatchKind::Single,
                    sub_bytes: vec![],
                })
            },

            // Block ADD Bank D (see 3.1.1j)
            [channel, 0x21, 0x00, 0x0A, 0x00, 0x02, tone_map @ ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::D),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Exp Bank E (see 3.1.1l)
            [channel, 0x21, 0x00, 0x0A, 0x00, 0x03, tone_map @ ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::E),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Exp Bank F (see 3.1.1n)
            [channel, 0x21, 0x00, 0x0A, 0x00, 0x04, tone_map @ ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: Some(BankIdentifier::F),
                    kind: PatchKind::Single,
                    sub_bytes: Vec::from(tone_map),
                })
            },

            // Block Multi/Combi (see 3.1.1h)
            [channel, 0x21, 0x00, 0x0A, 0x20, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::Block,
                    bank_identifier: None,
                    kind: PatchKind::Multi,
                    sub_bytes: vec![],
                })
            },

            // One drum kit (see 3.1.1e)
            [channel, 0x20, 0x00, 0x0A, 0x10, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::DrumKit,
                    sub_bytes: vec![],
                })
            },

            // One drum instrument (see 3.1.1g)
            [channel, 0x20, 0x00, 0x0A, 0x11, sub1, ..] => {
                Some(DumpCommand {
                    channel: *channel,
                    cardinality: Cardinality::One,
                    bank_identifier: None,
                    kind: PatchKind::DrumInstrument,
                    sub_bytes: vec![*sub1],
                })
            },

            // Block drum instrument (see 3.1.1f)
            [channel, 0x21, 0x00, 0x0A, 0x11, ..] => {
                Some(DumpCommand {
                    channel: *channel,
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
    fn size(&self) -> usize {
        1 + // channel     ("3rd" in K5000 MIDI spec)
        1 + // cardinality ("4th" in K5000 MIDI spec)
        1 + // 0x00 ("5th")
        1 + // 0x0A ("6th")
        1 + // kind ("7th")
        self.sub_bytes.len()  // 0 to max 19 (if block tone map present)
    }
}

impl fmt::Display for DumpCommand {
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
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
            DumpCommand::identify_vec(&cmd).unwrap(),
            DumpCommand {
                channel: 0,
                cardinality: Cardinality::Block,
                bank_identifier: None,
                kind: PatchKind::DrumInstrument,
                sub_bytes: vec![]
            }
        );
    }
}
