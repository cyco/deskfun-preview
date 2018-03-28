use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Result};

pub trait ReadVersionExt: io::Read {
    fn read_version(&mut self) -> Result<()> {
        self.read_u32::<LittleEndian>();

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadVersionExt for R {}
