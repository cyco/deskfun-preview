use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Result};

pub trait ReadNPCExt: io::Read {
    fn read_npc(&mut self) -> Result<()> {
        let character = self.read_u16::<LittleEndian>().unwrap();
        let x = self.read_u16::<LittleEndian>().unwrap();
        let y = self.read_u16::<LittleEndian>().unwrap();
        let unknown1 = self.read_i16::<LittleEndian>().unwrap();
        let unknown2 = self.read_i32::<LittleEndian>().unwrap();
        for _ in 0..0x20 {
            self.read_i8().unwrap();
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadNPCExt for R {}
