use super::super::game_type::GameType;
use super::marker::ReadMarkerExt;
use my_byte_order::ByteOrderExt;
use std::io;

pub trait ReadCharactersExt: io::Read {
    fn read_char_frame(&mut self) -> io::Result<()> {
        for _ in 0..0x8 {
            self.read_i16_le();
        }

        Ok(())
    }

    fn read_character(&mut self, game_type: GameType) -> io::Result<(i32, ())> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), ()));
        }

        self.read_category_marker("ICHA")?;
        let size = self.read_u32_le();
        let name = self.read_cstring_with_length(16)?;
        let char_type = self.read_i16_le();
        let movement_type = self.read_i16_le();
        if game_type == GameType::Yoda {
            let probably_garbage_1 = self.read_i16_le();
            let probably_garbage_2 = self.read_u32_le();
        }
        let frame1 = self.read_char_frame();
        let frame2 = self.read_char_frame();
        let frame3 = self.read_char_frame();

        Ok((index.into(), ()))
    }

    fn read_character_weapon(&mut self) -> io::Result<(i32, ())> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let reference = self.read_u16_le()?;
        let health = self.read_u16_le()?;

        Ok((index.into(), ()))
    }

    fn read_character_weapons(&mut self) -> io::Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character_weapon()? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_character_auxiliaries(&mut self) -> io::Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character_auxiliary()? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }

    fn read_character_auxiliary(&mut self) -> io::Result<(i32, ())> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let damage = self.read_u16_le()?;

        Ok((index.into(), ()))
    }

    fn read_characters(&mut self, game_type: GameType) -> io::Result<()> {
        self.read_u32_le();

        loop {
            match self.read_character(game_type)? {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadCharactersExt for R {}
