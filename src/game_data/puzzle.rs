use super::super::{GameType};
use super::marker::ReadMarkerExt;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use my_byte_order::ByteOrderExt;
use std::io;

pub trait ReadPuzzlesExt: io::Read {
    fn read_puzzle(&mut self, game_type: GameType) -> io::Result<(i32, ())> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), ()));
        }

        self.read_category_marker("IPUZ")?;
        let size = self.read_u32_le();
        if game_type == GameType::Yoda {
            let puzzle_type = self.read_u32_le();
        }
        let unknown1 = self.read_u32_le();
        let unknown2 = self.read_u32_le();
        let unknown3 = self.read_u16_le();

        for n in 0..5 {
            let length = self.read_u16_le()?;
            let mut buffer = vec![0; length as usize];
            self.read_exact(&mut buffer)?;
            let text = ISO_8859_1.decode(&buffer, DecoderTrap::Strict);
        }

        let item_1 = self.read_u16_le();
        if game_type == GameType::Yoda {
            let item_2 = self.read_u16_le();
        }

        Ok((index.into(), ()))
    }

    fn read_puzzles(&mut self, game_type: GameType) -> io::Result<()> {
        self.read_u32_le();

        loop {
            match self.read_puzzle(game_type)? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_puzzle_names(&mut self) -> io::Result<()> {
        let size = self.read_u32_le()? as usize;
        let mut buf = vec!(0; size);
        self.read_exact(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadPuzzlesExt for R {}
