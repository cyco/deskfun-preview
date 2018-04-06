use super::super::{GameType, CURRENT_GAME_TYPE};
use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadItemsExt: io::Read {
    fn read_tile_name(&mut self) -> Result<(i32, String)> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), "".to_string()));
        }

        let length = unsafe {
            if CURRENT_GAME_TYPE == GameType::Yoda {
                0x18
            } else {
                16
            }
        };

        let name = match self.read_cstring_with_length(length) {
            Ok(name) => name,
            Err(error) => String::new(),
        };

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
