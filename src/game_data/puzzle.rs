use super::super::GameType;
use super::marker::ReadMarkerExt;
use super::tile::TileId;
use byteorder::{ReadBytesExt, LE};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use std::io;

pub struct Puzzle {
    _id: usize,
    _name: String,
    _texts: [String; 5],
    _puzzle_type: u32,
    _item_1: TileId,
    _item_2: Option<TileId>,
}

pub trait ReadPuzzlesExt: io::Read {
    fn read_puzzle(&mut self, game_type: GameType) -> io::Result<Option<Puzzle>> {
        let index = self.read_i16::<LE>()?;
        if index == -1 {
            return Ok(None);
        }

        self.read_category_marker("IPUZ")?;
        let _size = self.read_u32::<LE>();
        if game_type == GameType::Yoda {
            let _puzzle_type = self.read_u32::<LE>();
        }
        let _unknown1 = self.read_u32::<LE>();
        let _unknown2 = self.read_u32::<LE>();
        let _unknown3 = self.read_u16::<LE>();

        let mut texts = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        for n in 0..5 {
            let length = self.read_u16::<LE>()?;
            let mut buffer = vec![0; length as usize];
            self.read_exact(&mut buffer)?;
            let text = match ISO_8859_1.decode(&buffer, DecoderTrap::Strict) {
                Err(why) => Err(io::Error::new(io::ErrorKind::Other, why)),
                Ok(text) => Ok(text),
            };
            texts[n] = text?;
        }

        let item_1 = self.read_u16::<LE>()?;
        let item_2 = None;
        if game_type == GameType::Yoda {
            match self.read_i16::<LE>()? {
                -1 => None,
                item_id => Some(item_id as u16),
            };
        };

        let puzzle = Puzzle {
            _id: 0,
            _name: String::new(),
            _puzzle_type: 0,
            _item_1: item_1,
            _item_2: item_2,

            _texts: texts,
        };

        Ok(Some(puzzle))
    }

    fn read_puzzles(&mut self, game_type: GameType) -> io::Result<Vec<Puzzle>> {
        let size = self.read_u32::<LE>()?;
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
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadPuzzlesExt for R {}
