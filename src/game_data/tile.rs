use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io;
use std::io::Read;

pub type Tiles = Vec<Tile>;

impl super::Category for Tiles {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let count = size / (32 * 32 + 4);

        for n in 0..count {
            let attributes = reader.read_u32::<LittleEndian>().unwrap();
            let mut pixels = Vec::new();
            reader.take(32 * 32).read_to_end(&mut pixels);
        }

        Ok(())
    }
}

pub struct Tile {}
impl Tile {}

impl super::Category for Tile {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let mut buffer = Vec::new();
        reader.take(size.into()).read_to_end(&mut buffer);

        Ok(())
    }
}
