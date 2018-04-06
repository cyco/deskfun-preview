use super::super::{GameType, CURRENT_GAME_TYPE};
use my_byte_order::ByteOrderExt;
use std::io::{self, Read};

pub trait ReadPuzzlesExt: io::Read {
    fn read_puzzle(&mut self) -> io::Result<(i32, ())> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(marker == "IPUZ", "Expected category marker IPUZ");
        let size = self.read_u32_le();
        unsafe {
            if CURRENT_GAME_TYPE == GameType::Yoda {
                let puzzle_type = self.read_u32_le();
            }
        }
        let unknown1 = self.read_u32_le();
        let unknown2 = self.read_u32_le();
        let unknown3 = self.read_u16_le();

        for n in 0..5 {
            let length = self.read_u16_le()?;
            let mut text = String::new();
            // self.take(length.into()).read_to_string(&mut text)?;
            match self.take(length.into()).read_to_string(&mut text) {
                Ok(length) => text,
                Err(err) => {println!("error: {}", err); String::new()},
            };
        }
        let item_1 = self.read_u16_le();
        unsafe {
            if CURRENT_GAME_TYPE == GameType::Yoda {
                let item_2 = self.read_u16_le();
            }
        }

        Ok((index.into(), ()))
    }

    fn read_puzzles(&mut self) -> io::Result<()> {
        self.read_u32_le();

        loop {
            match self.read_puzzle()? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_puzzle_names(&mut self) -> io::Result<()> {
        let size = self.read_u32_le()?;

        let mut buf = Vec::new();
        self.take(size as u64).read_to_end(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadPuzzlesExt for R {}
