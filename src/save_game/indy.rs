use super::SaveGameReading;
use byteorder::{ReadBytesExt, LE};
use std::io;

use super::super::game_data::zone::Zone;
use super::super::game_type::GameType;
use super::super::point::Point;

use super::save_game::SaveGame;

pub enum Indy {}

impl SaveGameReading for Indy {
    const MAGIC: &'static str = "INDYSAV44";

    fn read_save_game(buf: &mut io::Read, mut zones: &mut Vec<Zone>) -> io::Result<SaveGame> {
        let seed = buf.read_u32::<LE>()? & 0xFFFF;

        Self::read_puzzles(buf)?;
        Self::read_world(buf, &mut zones, GameType::Indy, 0..10, 0..10)?;
        Self::read_inventory(buf)?;

        let current_zone_id = buf.read_u16::<LE>()?;
        let pos_x_on_world = buf.read_u32::<LE>()?;
        let pos_y_on_world = buf.read_u32::<LE>()?;

        let mut point: Point = Point(0, 0);
        let u1 = buf.read_i16::<LE>()?;
        let u2 = buf.read_i16::<LE>()?;
        let u3 = buf.read_i16::<LE>()?;
        let u4 = buf.read_i16::<LE>()?;
        let u5 = buf.read_i16::<LE>()?;

        let u6 = buf.read_i16::<LE>()?;
        let u7 = buf.read_i16::<LE>()?;
        let u8 = buf.read_i16::<LE>()?;
        let u9 = buf.read_i16::<LE>()?;

        Ok(SaveGame {
            current_zone_id: current_zone_id,
            position_on_zone: point,
        })
    }

    fn read_world_item(buf: &mut io::Read, x: u8, y: u8) -> io::Result<()> {
        let visited = Self::read_bool(buf);
        let solved_1 = Self::read_bool(buf);
        let solved_2 = Self::read_bool(buf);

        let zone_id = buf.read_i16::<LE>()?;
        let field_c = buf.read_i16::<LE>()?;

        let required_item_id = buf.read_i16::<LE>()?;
        let find_item_id = buf.read_i16::<LE>()?;

        let npc_id = buf.read_i16::<LE>()?;
        // possibly zone or puzzle type
        let unkonwn = buf.read_i16::<LE>()?;

        Ok(())
    }

    fn read_int(buf: &mut io::Read) -> io::Result<i64> {
        Ok(buf.read_i16::<LE>()? as i64)
    }

    fn read_bool(buf: &mut io::Read) -> io::Result<bool> {
        Ok(buf.read_i16::<LE>()? != 0)
    }

    fn read_npc(buf: &mut io::Read) -> io::Result<()> {
        let mut data = vec![0; 0x20];
        buf.read_exact(&mut data)
    }
}