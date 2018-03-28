use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Result};

pub trait ReadPuzzlesExt: io::Read {
    fn read_puzzle(&mut self) -> Result<(i32, ())> {
        let index = self.read_i16::<LittleEndian>().unwrap();
        if index == -1 {
            return Ok((index.into(), ()));
        }

        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(marker == "IPUZ", "Expected category marker IPUZ");
        let size = self.read_u32::<LittleEndian>();
        let puzzle_type = self.read_u32::<LittleEndian>();
        let unknown1 = self.read_u32::<LittleEndian>();
        let unknown2 = self.read_u32::<LittleEndian>();
        let unknown3 = self.read_u16::<LittleEndian>();

        for n in 0..5 {
            let length = self.read_u16::<LittleEndian>().unwrap();
            let mut text = String::new();
            self.take(length.into()).read_to_string(&mut text);
        }
        let item_1 = self.read_u16::<LittleEndian>();
        let item_2 = self.read_u16::<LittleEndian>();

        Ok((index.into(), ()))
    }

    fn read_puzzles(&mut self) -> Result<()> {
        self.read_u32::<LittleEndian>();

        loop {
            match self.read_puzzle().unwrap() {
                (-1, _) => break,
                _ => (),
            }
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadPuzzlesExt for R {}
