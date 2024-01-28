// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::Bits;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Debug, Default, Clone)]
pub struct AsciiData {
    message: String,
}

impl Packet for AsciiData {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(self.message.clone().into_bytes())
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct AsciiDataBuilder;
pub static BUILDER: AsciiDataBuilder = AsciiDataBuilder;
impl PacketBuilder<AsciiData> for AsciiDataBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<AsciiData, Self::Error> {
        Ok(AsciiData {
            message: input.read_all_str_lossy()?,
        })
    }
}
