//! System Exclusive data definitions for K4.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use syxpack::{
    SystemExclusiveData,
    ParseError,
    MidiChannel,
    Ranged,
    Encoding,
};

const GROUP: u8 = 0x00;      // synth group
const MACHINE_ID: u8 = 0x04; // K4/K4r ID

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
    pub channel: MidiChannel,
    pub function: Function,
    pub substatus1: u8,
    pub substatus2: u8,
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
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        Ok(Header {
            channel: MidiChannel::new(data[0] as i32 + 1),
            function: Function::try_from(data[1]).unwrap(),
            substatus1: data[4],
            substatus2: data[5],
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let ch = self.channel.encode();
        vec![
            ch as u8,
            self.function as u8,
            GROUP,
            MACHINE_ID,
            self.substatus1,
            self.substatus2,
        ]
    }

    fn data_size() -> usize { 6 }
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
        let header = Header::parse(header_data);

        // The raw data is everything in the payload after the header.
        let raw_data = &payload[Header::data_size() as usize..];

        match (header.as_ref().unwrap().function, header.as_ref().unwrap().substatus1, header.as_ref().unwrap().substatus2) {
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
            (Function::OnePatchDataDump, 0x01, 32) =>
                Ok(Dump { kind: Kind::Drum, locality: Locality::Internal, payload: raw_data.to_vec() }),
            (Function::OnePatchDataDump, 0x03, 32) =>
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

#[cfg(test)]
mod tests {
    use super::{*};
    use syxpack::Message;

    static DATA: &'static [u8] = include_bytes!("A401.SYX");

    #[test]
    fn test_dump_identify_all() {
        match Message::parse(&DATA.to_vec()) {
            Ok(Message::ManufacturerSpecific { manufacturer: _, payload }) => {
                match Dump::identify(payload) {
                    Ok(dump) => {
                        assert_eq!(dump.kind, Kind::All);
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            },
            Ok(Message::Universal { kind: _, target: _, sub_id1: _, sub_id2: _, payload: _ }) => {
                panic!("Universal message, not manufacturer-specific");
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    #[test]
    fn test_dump_identify_single() {
        let data: [u8; 137] = [
            0x00, 0x20, 0x00, 0x04, 0x00, 0x00, 0x4d, 0x65,
            0x6c, 0x6f, 0x20, 0x56, 0x6f, 0x78, 0x20, 0x31, 0x64, 0x20,
            0x06, 0x04, 0x0c, 0x02, 0x1c, 0x3f, 0x39, 0x31, 0x32, 0x32,
            0x32, 0x3d, 0x00, 0x30, 0x00, 0x32, 0x32, 0x32, 0x00, 0x00,
            0x02, 0x03, 0x00, 0x00, 0x50, 0x40, 0x12, 0x12, 0x7e, 0x7f,
            0x4c, 0x4c, 0x5a, 0x5b, 0x00, 0x34, 0x02, 0x03, 0x2c, 0x37,
            0x34, 0x35, 0x02, 0x02, 0x15, 0x11, 0x4b, 0x4b, 0x34, 0x35,
            0x36, 0x36, 0x34, 0x35, 0x48, 0x48, 0x34, 0x35, 0x5a, 0x5a,
            0x34, 0x35, 0x40, 0x40, 0x02, 0x01, 0x41, 0x41, 0x35, 0x36,
            0x32, 0x32, 0x35, 0x36, 0x2c, 0x2c, 0x35, 0x36, 0x32, 0x32,
            0x35, 0x36, 0x32, 0x32, 0x35, 0x36, 0x32, 0x32, 0x33, 0x34,
            0x31, 0x51, 0x02, 0x07, 0x32, 0x34, 0x5b, 0x34, 0x32, 0x34,
            0x36, 0x34, 0x32, 0x33, 0x56, 0x01, 0x64, 0x02, 0x32, 0x63,
            0x56, 0x01, 0x32, 0x33, 0x32, 0x33, 0x32, 0x33, 0x6e
        ];
        
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
