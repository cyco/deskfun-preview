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

    fn read_save_game(buf: &mut dyn io::Read, mut zones: &mut Vec<Zone>) -> io::Result<SaveGame> {
        let _seed = buf.read_u32::<LE>()? & 0xFFFF;

        let _planet = buf.read_u32::<LE>()?;
        let _on_dagobah = buf.read_u32::<LE>()? != 0;

        Self::read_puzzles(buf)?;
        Self::read_puzzles(buf)?;
        Self::read_world(buf, &mut zones, GameType::Yoda, 4..6, 4..6)?;
        Self::read_world(buf, &mut zones, GameType::Yoda, 0..10, 0..10)?;
        Self::read_inventory(buf)?;

        let current_zone_id = buf.read_u16::<LE>()?;
        let _pos_x_on_world = buf.read_u32::<LE>()?;
        let _pos_y_on_world = buf.read_u32::<LE>()?;

        let current_weapon = buf.read_i16::<LE>()?;
        let _current_ammo = if current_weapon >= 0 {
            buf.read_i16::<LE>()?
        } else {
            -1
        };

        let _force_ammo = buf.read_i16::<LE>()?;
        let _blaster_ammo = buf.read_i16::<LE>()?;
        let _blaster_rifle_ammo = buf.read_i16::<LE>()?;

        let pos_x_on_zone = buf.read_u32::<LE>()? as usize / tile::WIDTH;
        let pos_y_on_zone = buf.read_u32::<LE>()? as usize / tile::HEIGHT;

        let _damage_taken = buf.read_u32::<LE>()?;
        let _lives_left = buf.read_u32::<LE>()?;
        let _difficulty = buf.read_u32::<LE>()?;
        let _time_elapsed = buf.read_u32::<LE>()?;

        let _world_size = buf.read_i16::<LE>()?;
        let _unknown_count = buf.read_i16::<LE>()?;
        let _unknown_sum = buf.read_i16::<LE>()?;
        let _unknown_thing = buf.read_i16::<LE>()?;

        let _goal_puzzle = buf.read_u32::<LE>()?;
        let _goal_puzzle_again = buf.read_u32::<LE>()?;

        let point: Point = Point(pos_x_on_zone as i64, pos_y_on_zone as i64);

        Ok(SaveGame {
            current_zone_id,
            position_on_zone: point,
        })
    }

    fn read_npc(buf: &mut dyn io::Read) -> io::Result<()> {
        let _character_id = buf.read_i16::<LE>()?;
        let _x = buf.read_i16::<LE>()?;
        let _y = buf.read_i16::<LE>()?;
        let _damage_taken = buf.read_i16::<LE>()?;
        let _enabled = Self::read_bool(buf)?;
        let _field_10 = buf.read_i16::<LE>()?;

        let _bullet_x = buf.read_i16::<LE>()?;
        let _bullet_y = buf.read_i16::<LE>()?;
        let _current_frame = buf.read_i16::<LE>()?;
        let _flag_18 = buf.read_u32::<LE>()?;
        let _flag_1c = buf.read_u32::<LE>()?;
        let _flag_20 = buf.read_u32::<LE>()?;
        let _direction_x = buf.read_i16::<LE>()?;
        let _direction_y = buf.read_i16::<LE>()?;

        let _bullet_offset = buf.read_i16::<LE>()?;
        let _facing_direction = buf.read_i16::<LE>()?;
        let _field_60 = buf.read_i16::<LE>()?;
        let _loot = buf.read_i16::<LE>()?;
        let _flag_2c = buf.read_u32::<LE>()?;
        let _flag_34 = buf.read_u32::<LE>()?;
        let _has_item = buf.read_u32::<LE>()?;
        let _cooldown = buf.read_i16::<LE>()?;
        let _preferred = buf.read_i16::<LE>()?;

        for _ in 0..4 {
            buf.read_u32::<LE>()?;
            buf.read_u32::<LE>()?;
        }

        Ok(())
    }

    fn read_sector(buf: &mut dyn io::Read, _x: u8, _y: u8) -> io::Result<()> {
        let _visited = Self::read_bool(buf);
        let _solved_1 = Self::read_bool(buf);
        let _solved_2 = Self::read_bool(buf);
        let _solved_3 = Self::read_bool(buf);
        let _solved_4 = Self::read_bool(buf);

        let _zone_id = buf.read_i16::<LE>()?;
        let _puzzle_index = buf.read_i16::<LE>()?;
        let _required_item_id = buf.read_i16::<LE>()?;
        let _find_item_id = buf.read_i16::<LE>()?;
        let _is_goal = buf.read_i16::<LE>()?;
        let _additional_required_item_id = buf.read_i16::<LE>()?;
        let _additional_gain_item = buf.read_i16::<LE>()?;
        let _npc_id = buf.read_i16::<LE>()?;

        let _unknown = buf.read_i32::<LE>()?;
        let _used_alternate_strain = buf.read_i16::<LE>()?;

        Ok(())
    }

    fn read_int(buf: &mut dyn io::Read) -> io::Result<i64> {
        Ok(buf.read_i32::<LE>()? as i64)
    }

    fn read_bool(buf: &mut dyn io::Read) -> io::Result<bool> {
        Ok(buf.read_i32::<LE>()? != 0)
    }
}
