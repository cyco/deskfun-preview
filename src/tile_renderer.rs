use super::game_data::tile::{Tile, HEIGHT, WIDTH};
use super::palette::{Color, Palette};

struct TileRenderer {
    palette: Palette,
    tiles: Vec<Tile>,
}

impl TileRenderer {
    fn render(&mut self, id: u16) -> [u8; WIDTH * HEIGHT * 4] {
        let pixels = self.tiles[id as usize].pixels;
        let mut result = [0; WIDTH * HEIGHT * 4];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                match self.palette.at(pixels[idx]) {
                    Color::Transparent => continue,
                    Color::RGB(r, g, b) => {
                        result[idx] = r;
                        result[idx + 1] = g;
                        result[idx + 2] = b;
                        result[idx + 3] = 0xFF;
                    }
                }
            }
        }

        result
    }
}
