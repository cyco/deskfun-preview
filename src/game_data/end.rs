use my_byte_order::ByteOrderExt;
use std::io;

pub trait ReadEndExt: io::Read {
    fn read_end(&mut self) -> io::Result<()> {
        self.read_u32_le()?;
        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadEndExt for R {}
