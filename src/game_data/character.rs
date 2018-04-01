use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub trait ReadCharactersExt: io::Read {
    fn read_char_frame(&mut self) -> Result<()> {
        for _ in 0..0x8 {
            self.read_i16_le();
        }

        Ok(())
    }

    fn read_character(&mut self) -> Result<(i32, ())> {
        let index = self.read_i16_le().unwrap();
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(marker == "ICHA", "Expected category marker ICHA");
        let size = self.read_u32_le();
        let mut name = String::new();
        self.take(16).read_to_string(&mut name);
        let char_type = self.read_i16_le();
        let movement_type = self.read_i16_le();
        let probably_garbage_1 = self.read_i16_le();
        let probably_garbage_2 = self.read_u32_le();
        let frame1 = self.read_char_frame();
        let frame2 = self.read_char_frame();
        let frame3 = self.read_char_frame();

        Ok((index.into(), ()))
    }

    fn read_character_weapon(&mut self) -> Result<(i32, ())> {
        let index = self.read_i16_le().unwrap();
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let reference = self.read_u16_le().unwrap();
        let health = self.read_u16_le().unwrap();

        Ok((index.into(), ()))
    }

    fn read_character_weapons(&mut self) -> Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character_weapon().unwrap() {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_character_auxiliaries(&mut self) -> Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character_auxiliary().unwrap() {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_character_auxiliary(&mut self) -> Result<(i32, ())> {
        let index = self.read_i16_le().unwrap();
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let damage = self.read_u16_le().unwrap();

        Ok((index.into(), ()))
    }

    fn read_characters(&mut self) -> Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character().unwrap() {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadCharactersExt for R {}
