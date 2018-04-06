use my_byte_order::ByteOrderExt;
use std::io;

pub struct NPC {}

pub trait ReadNPCExt: io::Read {
    fn read_npc(&mut self) -> io::Result<NPC> {
        let character = self.read_u16_le()?;
        let x = self.read_u16_le()?;
        let y = self.read_u16_le()?;
        let unknown1 = self.read_i16_le()?;
        let unknown2 = self.read_i32_le()?;

        let mut unknown = vec!(0 as u8; 0x20);
        self.read_exact(&mut unknown)?;

        Ok(NPC {})
    }
}

impl<R: io::Read + ?Sized> ReadNPCExt for R {}
