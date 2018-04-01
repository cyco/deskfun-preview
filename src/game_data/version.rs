use byteorder::{LittleEndian, ReadBytesExt};
use my_byte_order::ByteOrderExt;
use std::io::{self, Result};

pub trait ReadVersionExt: io::Read {
    fn read_version(&mut self) -> Result<()> {
        self.read_u32_le();

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadVersionExt for R {}
