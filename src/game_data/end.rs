use byteorder::{ReadBytesExt, LE};
use std::io;

pub trait ReadEndExt: io::Read {
    fn read_end(&mut self) -> io::Result<()> {
        self.read_u32::<LE>()?;
        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadEndExt for R {}
