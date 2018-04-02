use super::game_data::*;

struct ZoneRenderer {
    data: GameData,
}

impl ZoneRenderer {
    fn render(&self) -> [u8; 288 * 288 * 3] {
        let result = [0; 288 * 288 * 3];

        result
    }
}
