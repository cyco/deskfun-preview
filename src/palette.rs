use std::io;
use std::ops::Index;

pub struct Palette {
    data: [u8; 0x100],
}

pub enum Color {
    Transparent,
    RGB(u8, u8, u8),
}

impl Palette {
    pub fn new(input: &mut io::Read) -> io::Result<Palette> {
        let mut buffer = [0 as u8; 0x100];
        input.read_exact(&mut buffer)?;

        Ok(Palette { data: buffer })
    }

    pub fn at(&self, index: u8) -> Color {
        match index {
            0 => Color::Transparent,
            i => Color::RGB(4 * i + 2, 4 * i + 1, 4 * i + 0),
        }
    }
}
