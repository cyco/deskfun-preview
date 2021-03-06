use super::game_data::tile::{HEIGHT, WIDTH};
use super::game_data::GameData;
use super::game_type::GameType;
use super::palette::Palette;
use super::point::Point;
use super::tile_renderer::render_tile;
use std::cmp::{max, min};

pub fn render_zone(data: &GameData, palette: &Palette, zone_id: u16, hero: Point) -> Vec<u8> {
    let viewport_width: i64 = 9;
    let viewport_height: i64 = 9;
    let mut result = vec![
        0;
        viewport_width as usize
            * WIDTH as usize
            * viewport_height as usize
            * HEIGHT as usize
            * 4
    ];
    let zone = &data.zones[zone_id as usize];

    for y in 0..288 {
        for x in 0..288 {
            let idx = (y * 288 + x) * 4;
            result[idx + 0] = 0;
            result[idx + 1] = 0;
            result[idx + 2] = 0;
            result[idx + 3] = 0xFF;
        }
    }

    let x_offset = max(
        min(viewport_width / 2 - hero.0 as i64, 0),
        viewport_width - zone.width as i64,
    );
    let y_offset = max(
        min(viewport_height / 2 - hero.1 as i64, 0),
        viewport_height - zone.height as i64,
    );

    for y in 0..viewport_height as i64 {
        for x in 0..viewport_width as i64 {
            for z in 0..3 {
                if let Some(tile_id) =
                    zone.tile_id_at(((x - x_offset) as u8, (y - y_offset) as u8, z))
                {
                    render_tile(
                        data,
                        palette,
                        tile_id as u16,
                        x as usize,
                        y as usize,
                        viewport_width as usize * WIDTH as usize,
                        &mut result,
                    );
                }

                if x - x_offset == hero.0 && y - y_offset == hero.1 && z == 1 {
                    let hero = match data.game_type {
                        GameType::Yoda => 799,
                        GameType::Indy => 93,
                    };

                    render_tile(
                        data,
                        palette,
                        hero as u16,
                        x as usize,
                        y as usize,
                        viewport_width as usize * WIDTH as usize,
                        &mut result,
                    );
                }
            }
        }
    }

    result
}
