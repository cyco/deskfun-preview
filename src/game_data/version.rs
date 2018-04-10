use byteorder::{ReadBytesExt, LE};
use std::io;

pub trait ReadVersionExt: io::Read {
    fn read_version(&mut self) -> io::Result<()> {
        self.read_u32::<LE>();

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadVersionExt for R {}
