use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io::{self, Read, Result};

pub trait ReadEndExt: io::Read {
    fn read_end(&mut self) -> Result<()> {
        self.read_u32::<LittleEndian>().unwrap();
        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadEndExt for R {}
