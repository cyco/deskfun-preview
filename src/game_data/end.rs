use byteorder::{LittleEndian, ReadBytesExt};
use my_byte_order::ByteOrderExt;
use std::io::{self, Result};

pub trait ReadEndExt: io::Read {
    fn read_end(&mut self) -> Result<()> {
        self.read_u32_le().unwrap();
        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadEndExt for R {}
