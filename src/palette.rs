use std::io;

pub struct Palette {
    data: [u8; 0x400],
}

pub enum Color {
    Transparent,
    RGB(u8, u8, u8),
}

impl Palette {
    pub fn new(input: &mut io::Read) -> io::Result<Palette> {
        let mut buffer = [0; 0x400];
        input.read_exact(&mut buffer)?;

        Ok(Palette { data: buffer })
    }

    pub fn at(&self, index: u8) -> Color {
        match index as usize {
            0 => Color::Transparent,
            i => Color::RGB(
                self.data[4 * i + 2],
                self.data[4 * i + 1],
                self.data[4 * i + 0],
            ),
        }
    }
}
