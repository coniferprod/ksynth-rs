//! System Exclusive data definitions for K4.
//!

use std::convert::TryFrom;
use std::fmt;
use num_enum::TryFromPrimitive;
use crate::SystemExclusiveData;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Function {
    OnePatchDumpRequest = 0x00,
    BlockPatchDumpRequest = 0x01,
    AllPatchDumpRequest = 0x02,
    ParameterSend = 0x10,
    OnePatchDataDump = 0x20,
    BlockPatchDataDump = 0x21,
    AllPatchDataDump = 0x22,
    EditBufferDump = 0x23,
    ProgramChange = 0x30,
    WriteComplete = 0x40,
    WriteError = 0x41,
    WriteErrorProtect = 0x42,
    WriteErrorNoCard = 0x43,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Function::OnePatchDumpRequest => String::from("One Patch Dump Request"),
            Function::BlockPatchDumpRequest => String::from("Block Patch Dump Request"),
            Function::AllPatchDumpRequest => String::from("All Patch Dump Request"),
            Function::ParameterSend => String::from("Parameter Send"),
            Function::OnePatchDataDump => String::from("One Patch Data Dump"),
            Function::BlockPatchDataDump => String::from("Block Patch Data Dump"),
            Function::AllPatchDataDump => String::from("All Patch Data Dump"),
            Function::EditBufferDump => String::from("Edit Buffer Dump"),
            Function::ProgramChange => String::from("Program Change"),
            Function::WriteComplete => String::from("Write Complete"),
            Function::WriteError => String::from("Write Error"),
            Function::WriteErrorProtect => String::from("Write Error (Protect)"),
            Function::WriteErrorNoCard => String::from("Write Error (No Card)"),
        })
    }
}

/// K4 System Exclusive Message header
pub struct Header {
    pub channel: u8,
    pub function: Function,
    pub group: u8,
    pub machine_id: u8,
    pub substatus1: u8,
    pub substatus2: u8,
}

impl Header {
    pub fn data_size() -> u32 {
        6
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ch: {}  Fn: {}, Sub1: {}, Sub2: {}",
            self.channel,
            self.function,
            self.substatus1,
            self.substatus2)
    }
}

impl SystemExclusiveData for Header {
    fn from_bytes(data: Vec<u8>) -> Self {
        Header {
            channel: data[0] + 1,
            function: Function::try_from(data[1]).unwrap(),
            group: data[2],
            machine_id: data[3],
            substatus1: data[4],
            substatus2: data[5],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.channel - 1,  // 1...16 to 0...15
            self.function as u8,
            self.group,
            self.machine_id,
            self.substatus1,
            self.substatus2,
        ]
    }
}

pub enum Locality {
    Internal,
    External,
}

impl fmt::Display for Locality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Locality::Internal => String::from("INT"),
            Locality::External => String::from("EXT"),
        })
    }
}

pub struct Dump {
    pub kind: Kind,
    pub locality: Locality,
    pub payload: Vec<u8>,
}

/// Represents the kind of Kawai K4 MIDI System Exclusive dump.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Kind {
    All,

    // u8: number for single A-1 ~ D-16
    // Locality: INT or EXT
    // Vec<u8>: the raw data
    OneSingle(u8),

    // Int: number for multi A-1 ~ D-16
    // Locality: INT or EXT
    // ByteArray: the raw data
    OneMulti(u8),

    Drum,
    OneEffect(u8),
    BlockSingle,
    BlockMulti,
    BlockEffect,
}

impl Dump {
    /// Identifies the SysEx message and returns the corresponding
    /// enumeration value with the raw data.
    pub fn identify(payload: Vec<u8>) -> Result<Dump, ParseError> {
        // Extract the SysEx header from the message payload:

        let header_data = &payload[0..Header::data_size() as usize];
        let header = Header::from_bytes(header_data.to_vec());

        // The raw data is everything in the payload after the header.
        let raw_data = &payload[Header::data_size() as usize..];

        match (header.function, header.substatus1, header.substatus2) {
            (Function::OnePatchDataDump, 0x00, number) if (0..=63).contains(&number) =>
                Ok(Dump { kind: Kind::OneSingle(number), locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x00, number) if (64..=127).contains(&number) =>
                Ok(Dump { kind: Kind::OneMulti(number), locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x02, number) if (0..=63).contains(&number) =>
                Ok(Dump { kind: Kind::OneSingle(number), locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x02, number) if (64..=127).contains(&number) =>
                Ok(Dump { kind: Kind::OneMulti(number), locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x01, number) if (0..=31).contains(&number) =>
                Ok(Dump { kind: Kind::OneEffect(number), locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x03, number) if (0..=31).contains(&number) =>
                Ok(Dump { kind: Kind::OneEffect(number), locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x01, number) if number == 32 =>
                Ok(Dump { kind: Kind::Drum, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x03, number) if number == 32 =>
                Ok(Dump { kind: Kind::Drum, locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x00, 0x00) =>
                Ok(Dump { kind: Kind::BlockSingle, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x00, 0x40) =>
                Ok(Dump { kind: Kind::BlockMulti, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x02, 0x00) =>
                Ok(Dump { kind: Kind::BlockSingle, locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x02, 0x40) =>
                Ok(Dump { kind: Kind::BlockMulti, locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x01, 0x00) =>
                Ok(Dump { kind: Kind::BlockEffect, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::BlockPatchDataDump, 0x03, 0x00) =>
                Ok(Dump { kind: Kind::BlockEffect, locality: Locality::External, payload: raw_data.to_vec() }),
            (Function::AllPatchDataDump, 0x00, 0x00) =>
                Ok(Dump { kind: Kind::All, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::AllPatchDataDump, 0x02, 0x00) =>
                Ok(Dump { kind: Kind::All, locality: Locality::External, payload: raw_data.to_vec() }),
            _ => Err(ParseError::Unidentified),

        }
    }
}

/// Error type for parsing data from MIDI System Exclusive bytes.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ParseError {
    NotEnoughData(u32, u32),  // actual, expected
    BadChecksum(u8, u8),  // actual, expected
    InvalidData(u32),  // offset in data
    Unidentified,  // can't identify this kind
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ParseError::NotEnoughData(actual, expected) => format!("Got {} bytes of data, expected {} bytes.", actual, expected),
            ParseError::BadChecksum(actual, expected) => format!("Computed checksum was {}H, expected {}H.", actual, expected),
            ParseError::InvalidData(offset) => format!("Invalid data at offset {}.", offset),
            ParseError::Unidentified => String::from("Unable to identify this System Exclusive file."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{*};
    use syxpack::Message;

    #[test]
    fn test_dump_identify_all() {
        let data: [u8; 15123] = include!("a401.in");
        match Message::new(&data.to_vec()) {
            Ok(Message::ManufacturerSpecific { manufacturer, payload }) => {
                match Dump::identify(payload) {
                    Ok(dump) => {
                        assert_eq!(dump.kind, Kind::All);
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            },
            Ok(Message::Universal { kind, target, sub_id1, sub_id2, payload }) => {
                panic!("Universal message, not manufacturer-specific");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    #[test]
    fn test_dump_identify_single() {
        let data: [u8; 137] = include!("intsingle.in");
        match Dump::identify(data.to_vec()) {
            Ok(dump) => {
                assert_eq!(dump.kind, Kind::OneSingle(0));
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
