use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadSoundExt: io::Read {
    fn read_sounds(&mut self) -> Result<()> {
        let size = self.read_u32_le()?;
        let count = -self.read_i16_le()?;

        for n in 0..count {
            let size = self.read_u16_le()?;
            let mut string = String::new();
            self.take(size.into())
                .read_to_string(&mut string)
                .expect("Unable to read sound name!");
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSoundExt for R {}
