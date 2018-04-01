use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadSetupImageExt: io::Read {
    fn read_setup_image(&mut self) -> Result<()> {
        let size = self.read_u32_le()?;
        let mut buffer = Vec::new();
        self.take(size.into()).read_to_end(&mut buffer)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSetupImageExt for R {}
