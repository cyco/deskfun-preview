use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadItemsExt: io::Read {
    fn read_tile_name(&mut self) -> Result<(i32, String)> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), "".to_string()));
        }

        let mut name = String::new();
        let mut name_region = self.take(0x18);
        name_region.read_to_string(&mut name);
        let mut garbage = Vec::new();
        name_region.read_to_end(&mut garbage);

        Ok((index.into(), name.to_string()))
    }

    fn read_tile_names(&mut self) -> Result<()> {
        self.read_u32_le();

        loop {
            match self.read_tile_name()? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadItemsExt for R {}
