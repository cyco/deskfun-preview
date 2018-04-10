use byteorder::{ReadBytesExt, LE};
use std::io;

#[derive(Clone)]
pub struct NPC {}

pub trait ReadNPCExt: io::Read {
    fn read_npc(&mut self) -> io::Result<NPC> {
        let character = self.read_u16::<LE>()?;
        let x = self.read_u16::<LE>()?;
        let y = self.read_u16::<LE>()?;
        let unknown1 = self.read_i16::<LE>()?;
        let unknown2 = self.read_i32::<LE>()?;

        let mut unknown = vec!(0 as u8; 0x20);
        self.read_exact(&mut unknown)?;

        Ok(NPC {})
    }
}

impl<R: io::Read + ?Sized> ReadNPCExt for R {}
