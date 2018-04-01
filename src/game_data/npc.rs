use my_byte_order::ByteOrderExt;
use std::io::{self, Result};

pub struct NPC {}

pub trait ReadNPCExt: io::Read {
    fn read_npc(&mut self) -> Result<NPC> {
        let character = self.read_u16_le().unwrap();
        let x = self.read_u16_le().unwrap();
        let y = self.read_u16_le().unwrap();
        let unknown1 = self.read_i16_le().unwrap();
        let unknown2 = self.read_i32_le().unwrap();
        for _ in 0..0x20 {
            self.read_i8_le().unwrap();
        }

        Ok(NPC {})
    }
}

impl<R: io::Read + ?Sized> ReadNPCExt for R {}
