use super::SaveGameReading;
use byteorder::{ReadBytesExt, LE};
use std::io;

use super::super::game_data::tile::{HEIGHT, WIDTH};
use super::super::game_data::zone::Zone;
use super::super::game_type::GameType;
use super::super::point::Point;

use super::save_game::SaveGame;

pub enum Indy {}

impl SaveGameReading for Indy {
    const MAGIC: &'static str = "INDYSAV44";

    fn read_save_game(buf: &mut dyn io::Read, mut zones: &mut Vec<Zone>) -> io::Result<SaveGame> {
        let _seed = buf.read_u32::<LE>()? & 0xFFFF;

        Self::read_puzzles(buf)?;
        Self::read_world(buf, &mut zones, GameType::Indy, 0..10, 0..10)?;
        Self::read_inventory(buf)?;

        let current_zone_id = buf.read_u16::<LE>()?;
        let _pos_x_on_world = buf.read_u16::<LE>()?;
        let _pos_y_on_world = buf.read_u16::<LE>()?;

        let _unknown1 = buf.read_u16::<LE>()?;
        let x = buf.read_u16::<LE>()?;
        let y = buf.read_u16::<LE>()?;
        let _u2 = buf.read_i16::<LE>()?;
        let _u3 = buf.read_i16::<LE>()?;
        let _u4 = buf.read_i16::<LE>()?;
        let _u5 = buf.read_i16::<LE>()?;

        let _u6 = buf.read_i16::<LE>()?;
        let _u7 = buf.read_i16::<LE>()?;
        let _u8 = buf.read_i16::<LE>()?;
        let _u9 = buf.read_i16::<LE>()?;

        Ok(SaveGame {
            current_zone_id,
            position_on_zone: Point((x / WIDTH as u16) as i64, (y / HEIGHT as u16) as i64),
        })
    }

    fn read_world_item(buf: &mut dyn io::Read, _x: u8, _y: u8) -> io::Result<()> {
        let _visited = Self::read_bool(buf);
        let _solved_1 = Self::read_bool(buf);
        let _solved_2 = Self::read_bool(buf);

        let _zone_id = buf.read_i16::<LE>()?;
        let _field_c = buf.read_i16::<LE>()?;

        let _required_item_id = buf.read_i16::<LE>()?;
        let _find_item_id = buf.read_i16::<LE>()?;

        let _npc_id = buf.read_i16::<LE>()?;
        // possibly zone or puzzle type
        let _unkonwn = buf.read_i16::<LE>()?;

        Ok(())
    }

    fn read_int(buf: &mut dyn io::Read) -> io::Result<i64> {
        Ok(buf.read_i16::<LE>()? as i64)
    }

    fn read_bool(buf: &mut dyn io::Read) -> io::Result<bool> {
        Ok(buf.read_i16::<LE>()? != 0)
    }

    fn read_npc(buf: &mut dyn io::Read) -> io::Result<()> {
        let mut data = vec![0; 0x20];
        buf.read_exact(&mut data)
    }
}
