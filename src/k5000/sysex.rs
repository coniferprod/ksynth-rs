//! System Exclusive data definitions.
//!

use std::convert::TryFrom;
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
