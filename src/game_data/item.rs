use super::super::{GameType};
use byteorder::{ReadBytesExt, LE};
use super::super::io::ByteOrderExt;
use std::io;

pub trait ReadItemsExt: io::Read {
    fn read_tile_name(&mut self, game_type: GameType) -> io::Result<(i32, String)> {
        let index = self.read_i16::<LE>()?;
        if index == -1 {
            return Ok((index.into(), "".to_string()));
        }

        let length = if game_type == GameType::Yoda {
            0x18
        } else {
            16
        };

        let name = self.read_iso_cstring_with_length(length)?;
        Ok((index.into(), name))
    }

    fn read_tile_names(&mut self, game_type: GameType) -> io::Result<()> {
        self.read_u32::<LE>();

        loop {
            match self.read_tile_name(game_type)? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadItemsExt for R {}
