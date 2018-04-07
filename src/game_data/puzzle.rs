use super::super::GameType;
use super::marker::ReadMarkerExt;
use super::tile::TileId;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use my_byte_order::ByteOrderExt;
use std::io;

pub struct Puzzle {
    id: usize,
    name: String,
    texts: [String; 5],
    puzzle_type: u32,
    item_1: TileId,
    item_2: Option<TileId>,
}

pub trait ReadPuzzlesExt: io::Read {
    fn read_puzzle(&mut self, game_type: GameType) -> io::Result<Option<Puzzle>> {
        let index = self.read_i16_le()?;
        if index == -1 {
            return Ok(None);
        }

        self.read_category_marker("IPUZ")?;
        let size = self.read_u32_le();
        if game_type == GameType::Yoda {
            let puzzle_type = self.read_u32_le();
        }
        let unknown1 = self.read_u32_le();
        let unknown2 = self.read_u32_le();
        let unknown3 = self.read_u16_le();

        let mut texts = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        for n in 0..5 {
            let length = self.read_u16_le()?;
            let mut buffer = vec![0; length as usize];
            self.read_exact(&mut buffer)?;
            let text = match ISO_8859_1.decode(&buffer, DecoderTrap::Strict) {
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why)),
                Ok(text) => Ok(text),
            };
            texts[n] = text?;
        }

        let item_1 = self.read_u16_le()?;
        let item_2 = None;
        if game_type == GameType::Yoda {
            match self.read_i16_le()? {
                -1 => None,
                item_id => Some(item_id as u16),
            };
        };

        let puzzle = Puzzle {
            id: 0,
            name: String::new(),
            puzzle_type: 0,
            item_1: item_1,
            item_2: item_2,

            texts: texts,
        };
        
        Ok(Some(puzzle))
    }

    fn read_puzzles(&mut self, game_type: GameType) -> io::Result<Vec<Puzzle>> {
        let size = self.read_u32_le()?;
        let estimated_count = size / 24;

        let mut result = Vec::with_capacity(estimated_count as usize);
        loop {
            match self.read_puzzle(game_type)? {
                None => break,
                Some(puzzle) => result.push(puzzle),
            }
        }

        Ok(result)
    }

    fn read_puzzle_names(&mut self) -> io::Result<()> {
        let size = self.read_u32_le()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadPuzzlesExt for R {}
