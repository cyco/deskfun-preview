use super::game_data::GameData;
use super::game_data::tile::{HEIGHT, WIDTH};
use super::palette::Palette;
use super::point::Point;
use super::tile_renderer::TileRenderer;
use super::game_type::GameType;

pub struct ZoneRenderer {
    data: GameData,
    palette: Palette,
}

impl ZoneRenderer {
    pub fn new(data: GameData, palette: Palette) -> ZoneRenderer {
        ZoneRenderer {
            palette: palette,
            data: data,
        }
    }

    pub fn render(self, zone_id: u16, hero: Point) -> Vec<u8> {
        let tile_renderer = TileRenderer::new(self.data.tiles, self.palette);

        let mut result = vec![0; 9 * WIDTH * 9 * HEIGHT * 4];
        let zone = &self.data.zones[zone_id as usize];

        for y in 0..288 {
            for x in 0..288 {
                let idx = (y * 288 + x) * 4;
                result[idx + 0] = 0;
                result[idx + 1] = 0;
                result[idx + 2] = 0;
                result[idx + 3] = 0xFF;
            }
        }

        for y in 0..9 {
            for x in 0..9 {
                for z in 0..3 {
                    if let Some(tile_id) = zone.tile_id_at((x, y, z)) {
                        tile_renderer.render(
                            tile_id as u16,
                            x as usize,
                            y as usize,
                            9 * WIDTH,
                            &mut result,
                        );
                    }

                    if x == hero.0 as u8 && y == hero.1 as u8 && z == 1 {
                        let hero = match self.data.game_type {
                            GameType::Yoda => 799,
                            GameType::Indy => 0
                        };

                        tile_renderer.render(
                            hero as u16,
                            x as usize,
                            y as usize,
                            9 * WIDTH,
                            &mut result,
                        );
                    }
                }
            }
        }

        result
    }
}
