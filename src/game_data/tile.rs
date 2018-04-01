use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadTileExt: io::Read {
    fn read_tiles(&mut self) -> Result<()> {
        let size = self.read_u32_le()?;
        let count = size / (32 * 32 + 4);

        for n in 0..count {
            let attributes = self.read_u32_le()?;
            let mut pixels = Vec::new();
            self.take(32 * 32).read_to_end(&mut pixels);
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadTileExt for R {}
