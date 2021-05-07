use super::game_data::tile::{HEIGHT, WIDTH};
use super::game_data::GameData;
use super::palette::{Color, Palette};

pub fn render_tile(
    data: &GameData,
    palette: &Palette,
    id: u16,
    x: usize,
    y: usize,
    width: usize,
    buffer: &mut Vec<u8>,
) -> Option<()> {
    let pixels = data.tiles[id as usize].pixels;
    let buffer_start = (HEIGHT * y * width) + x * WIDTH;

    for ty in 0..HEIGHT {
        for tx in 0..WIDTH {
            let tile_pixel = ty * WIDTH + tx;
            let idx = buffer_start + ty * width + tx;

            match palette.at(pixels[tile_pixel]) {
                Color::Transparent => continue,
                Color::RGB(r, g, b) => {
                    buffer[4 * idx] = r;
                    buffer[4 * idx + 1] = g;
                    buffer[4 * idx + 2] = b;
                    buffer[4 * idx + 3] = 0xFF;
                }
            }
        }
    }

    None
}
