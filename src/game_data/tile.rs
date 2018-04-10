use byteorder::{ReadBytesExt, LE};
use std::io;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 32;

pub type TileId = u16;

pub struct Tile {
    pub attributes: u32,
    pub pixels: [u8; WIDTH * HEIGHT],
}

pub trait ReadTileExt: io::Read {
    fn read_tiles(&mut self) -> io::Result<Vec<Tile>> {
        let size = self.read_u32::<LE>()? as usize;
        let count = size / (WIDTH * HEIGHT + 4);
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            result.push(Tile {
                attributes: self.read_u32::<LE>()?,
                pixels: {
                    let mut pixels = [0 as u8; WIDTH * HEIGHT];
                    self.read_exact(&mut pixels)?;
                    pixels
                },
            });
        }

        Ok(result)
    }
}

impl<R: io::Read + ?Sized> ReadTileExt for R {}
