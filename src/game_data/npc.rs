use byteorder::{ReadBytesExt, LE};
use std::io;

#[derive(Clone)]
pub struct NPC {}

pub trait ReadNPCExt: io::Read {
    fn read_npc(&mut self) -> io::Result<NPC> {
        let _character = self.read_u16::<LE>()?;
        let _x = self.read_u16::<LE>()?;
        let _y = self.read_u16::<LE>()?;
        let _unknown1 = self.read_i16::<LE>()?;
        let _unknown2 = self.read_i32::<LE>()?;

        let mut unknown = vec![0 as u8; 0x20];
        self.read_exact(&mut unknown)?;

        Ok(NPC {})
    }
}

impl<R: io::Read + ?Sized> ReadNPCExt for R {}
