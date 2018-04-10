use super::SaveGameReading;
use byteorder::{ReadBytesExt, LE};
use std::io;

use super::super::game_data::tile;
use super::super::game_data::zone::Zone;
use super::super::game_type::GameType;
use super::super::point::Point;

use super::save_game::SaveGame;

pub enum Yoda {}

impl SaveGameReading for Yoda {
    const MAGIC: &'static str = "YODASAV44";

    fn read_save_game(buf: &mut io::Read, mut zones: &mut Vec<Zone>) -> io::Result<SaveGame> {
        let seed = buf.read_u32::<LE>()? & 0xFFFF;

        let planet = buf.read_u32::<LE>()?;
        let on_dagobah = buf.read_u32::<LE>()? != 0;

        Self::read_puzzles(buf)?;
        Self::read_puzzles(buf)?;
        Self::read_world(buf, &mut zones, GameType::Yoda, 4..6, 4..6)?;
        Self::read_world(buf, &mut zones, GameType::Yoda, 0..10, 0..10)?;
        Self::read_inventory(buf)?;

        let current_zone_id = buf.read_u16::<LE>()?;
        let pos_x_on_world = buf.read_u32::<LE>()?;
        let pos_y_on_world = buf.read_u32::<LE>()?;

        let mut point: Point = Point(0, 0);
        let current_weapon = buf.read_i16::<LE>()?;
        let currentAmmo = if current_weapon >= 0 {
            buf.read_i16::<LE>()?
        } else {
            -1
        };

        let force_ammo = buf.read_i16::<LE>()?;
        let blaster_ammo = buf.read_i16::<LE>()?;
        let blaster_rifle_ammo = buf.read_i16::<LE>()?;

        let pos_x_on_zone = buf.read_u32::<LE>()? as usize / tile::WIDTH;
        let pos_y_on_zone = buf.read_u32::<LE>()? as usize / tile::HEIGHT;

        let damage_taken = buf.read_u32::<LE>()?;
        let lives_left = buf.read_u32::<LE>()?;
        let difficulty = buf.read_u32::<LE>()?;
        let time_elapsed = buf.read_u32::<LE>()?;

        let world_size = buf.read_i16::<LE>()?;
        let unknown_count = buf.read_i16::<LE>()?;
        let unknown_sum = buf.read_i16::<LE>()?;
        let unknown_thing = buf.read_i16::<LE>()?;

        let goal_puzzle = buf.read_u32::<LE>()?;
        let goal_puzzle_again = buf.read_u32::<LE>()?;

        point = Point(pos_x_on_zone as i64, pos_y_on_zone as i64);

        Ok(SaveGame {
            current_zone_id: current_zone_id,
            position_on_zone: point,
        })
    }

    fn read_npc(buf: &mut io::Read) -> io::Result<()> {
        let character_id = buf.read_i16::<LE>()?;
        let x = buf.read_i16::<LE>()?;
        let y = buf.read_i16::<LE>()?;
        let field_a = buf.read_i16::<LE>()?;
        let enabled = buf.read_u32::<LE>()? != 0;
        let field_10 = buf.read_i16::<LE>()?;
        let field_x__ = buf.read_i16::<LE>()?;
        let field_y__ = buf.read_i16::<LE>()?;
        let current_frame = buf.read_i16::<LE>()?;
        let field_18 = buf.read_u32::<LE>()?;
        let field_1c = buf.read_u32::<LE>()?;
        let field_2 = buf.read_u32::<LE>()?;
        let field_x_ = buf.read_i16::<LE>()?;
        let field_y_ = buf.read_i16::<LE>()?;
        let field_3c = buf.read_i16::<LE>()?;
        let field_3e = buf.read_i16::<LE>()?;
        let field_60 = buf.read_i16::<LE>()?;
        let field_26 = buf.read_i16::<LE>()?;
        let field_2c = buf.read_u32::<LE>()?;
        let field_34 = buf.read_u32::<LE>()?;
        let field_28 = buf.read_u32::<LE>()?;

        let field_24 = buf.read_i16::<LE>()?;
        let unknown = buf.read_i16::<LE>()?;

        for _ in 0..4 {
            buf.read_u32::<LE>()?;
            buf.read_u32::<LE>()?;
        }

        Ok(())
    }

    fn read_world_item(buf: &mut io::Read, x: u8, y: u8) -> io::Result<()> {
        let visited = Self::read_bool(buf);
        let solved_1 = Self::read_bool(buf);
        let solved_2 = Self::read_bool(buf);

        let solved_3 = buf.read_u32::<LE>()? != 0;
        let solved_4 = buf.read_u32::<LE>()? != 0;

        let zone_id = buf.read_i16::<LE>()?;
        let field_c = buf.read_i16::<LE>()?;
        let required_item_id = buf.read_i16::<LE>()?;
        let find_item_id = buf.read_i16::<LE>()?;
        let field_ea = buf.read_i16::<LE>()?;
        let additional_required_item_id = buf.read_i16::<LE>()?;
        let field_16 = buf.read_i16::<LE>()?;
        let npc_id = buf.read_i16::<LE>()?;

        let unknown_1 = buf.read_i32::<LE>()?;
        let unknown_2 = buf.read_i16::<LE>()?;

        Ok(())
    }

    fn read_int(buf: &mut io::Read) -> io::Result<i64> {
        Ok(buf.read_i32::<LE>()? as i64)
    }

    fn read_bool(buf: &mut io::Read) -> io::Result<bool> {
        Ok(buf.read_i32::<LE>()? != 0)
    }
}
