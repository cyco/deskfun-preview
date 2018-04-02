use super::game_data::tile::{Tile, HEIGHT, WIDTH};
use super::palette::{Color, Palette};

pub struct TileRenderer {
    palette: Palette,
    tiles: Vec<Tile>,
}

impl TileRenderer {
    pub fn new(tiles: Vec<Tile>, palette: Palette) -> TileRenderer {
        TileRenderer {
            palette: palette,
            tiles: tiles,
        }
    }

    pub fn render(&self, id: u16) -> [u8; WIDTH * HEIGHT * 4] {
        let pixels = self.tiles[id as usize].pixels;
        let mut result = [0; WIDTH * HEIGHT * 4];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                match self.palette.at(pixels[idx]) {
                    Color::Transparent => continue,
                    Color::RGB(r, g, b) => {
                        result[4 * idx] = r;
                        result[4 * idx + 1] = g;
                        result[4 * idx + 2] = b;
                        result[4 * idx + 3] = 0xFF;
                    }
                }
            }
        }

        result
    }
}
