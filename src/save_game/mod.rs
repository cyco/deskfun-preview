mod indy;
mod read_ext;
mod save_game;
mod yoda;
use byteorder::{ReadBytesExt, LE};
use std::io;
use std::ops::Range;

pub use self::read_ext::ReadSaveGameExt;
pub use self::save_game::*;

use super::game_data::hotspot::{Hotspot, HotspotType};
use super::game_data::npc::NPC;
use super::game_data::zone;
use super::game_data::zone::Zone;
use super::game_type::GameType;

type PuzzleId = u16;
type TileId = u16;

pub trait SaveGameReading {
    const MAGIC: &'static str;

    fn read_int(buf: &mut io::Read) -> io::Result<i64>;
    fn read_bool(buf: &mut io::Read) -> io::Result<bool>;

    fn read_puzzles(buf: &mut io::Read) -> io::Result<Vec<PuzzleId>> {
        let puzzle_id_count_1 = buf.read_u16::<LE>()?;
        let mut puzzle_ids_1 = Vec::with_capacity(puzzle_id_count_1 as usize);
        for _ in 0..puzzle_id_count_1 {
            puzzle_ids_1.push(buf.read_u16::<LE>()?);
        }
        Ok(puzzle_ids_1)
    }

    fn read_save_game(buf: &mut io::Read, zones: &mut Vec<Zone>) -> io::Result<SaveGame>;
    fn read_inventory(mut buf: &mut io::Read) -> io::Result<Vec<TileId>> {
        let count = Self::read_int(&mut buf)?;

        let mut item_ids = Vec::with_capacity(count as usize);
        for _ in 0..count {
            item_ids.push(buf.read_u16::<LE>()?);
        }

        Ok(item_ids)
    }

    fn read_world(
        buf: &mut io::Read,
        mut zones: &mut Vec<Zone>,
        game_type: GameType,
        x_range: Range<u8>,
        y_range: Range<u8>,
    ) -> io::Result<()> {
        for y in y_range {
            for x in x_range.clone() {
                Self::read_world_item(buf, x, y)?;
            }
        }

        Self::read_world_details(buf, &mut zones, game_type)?;
        Ok(())
    }

    fn read_world_item(buf: &mut io::Read, x: u8, y: u8) -> io::Result<()>;

    fn read_world_details(
        buf: &mut io::Read,
        mut zones: &mut Vec<Zone>,
        game_type: GameType,
    ) -> io::Result<()> {
        let mut x: i64;
        let mut y: i64;

        loop {
            y = Self::read_int(buf)?;
            x = Self::read_int(buf)?;

            if x == -1 || y == -1 {
                break;
            }

            let zone_id = buf.read_i16::<LE>()?;
            let visited = Self::read_bool(buf)?;

            Self::read_room(buf, zone_id, visited, &mut zones, game_type)?;
        }

        Ok(())
    }

    fn read_rooms(
        buf: &mut io::Read,
        zone_id: i16,
        mut zones: &mut Vec<Zone>,
        mut start: usize,
        game_type: GameType,
    ) -> io::Result<()> {
        let count: usize;
        let mut zone_ids = Vec::new();
        {
            let zone = &mut zones[zone_id as usize];
            let hotspots = &mut zone.hotspots;
            count = hotspots.len();

            for i in start..count {
                start = i;
                let door;
                {
                    let hotspot = &hotspots[i];
                    if let HotspotType::DoorIn = hotspot.hotspot_type {
                        if hotspot.argument == -1 {
                            continue;
                        }

                        door = hotspot.argument;
                    } else {
                        continue;
                    }
                }

                let zone_id = buf.read_i16::<LE>()?;
                let visited = Self::read_bool(buf)?;

                assert!(
                    zone_id == door,
                    "Expected door to lead to zone {} instead of {}",
                    zone_id,
                    door
                );
                zone_ids.push((door, visited));
                break;
            }
        }

        for (zone_id, visited) in zone_ids {
            Self::read_room(buf, zone_id, visited, &mut zones, game_type)?;
        }

        if start + 1 < count {
            Self::read_rooms(buf, zone_id, &mut zones, start + 1, game_type)?;
        }

        Ok(())
    }

    fn read_room(
        buf: &mut io::Read,
        zone_id: i16,
        visited: bool,
        mut zones: &mut Vec<Zone>,
        game_type: GameType,
    ) -> io::Result<()> {
        {
            let mut zone: &mut Zone = &mut zones[zone_id as usize];
            Self::read_zone(buf, &mut zone, visited, game_type)?;
        }

        Self::read_rooms(buf, zone_id, &mut zones, 0, game_type)?;
        Ok(())
    }

    fn read_zone(
        buf: &mut io::Read,
        mut zone: &mut Zone,
        visited: bool,
        game_type: GameType,
    ) -> io::Result<()> {
        if visited {
            let counter = Self::read_int(buf)?;
            let random = Self::read_int(buf)?;
            let doorInX = Self::read_int(buf)?;
            let doorInY = Self::read_int(buf)?;

            if game_type == GameType::Yoda {
                let padding = buf.read_u16::<LE>()?;
                let planet = buf.read_i16::<LE>()?;
            }

            let tile_count = zone.width as usize * zone.height as usize * zone::LAYERS as usize;
            let mut tile_ids = vec!(0 as i16; tile_count);
            buf.read_i16_into::<LE>(&mut tile_ids)?;
            zone.tiles = tile_ids;
        }

        let __visited = Self::read_bool(buf)?;
        Self::read_hotspots(buf, &mut zone, game_type)?;

        if visited {
            Self::read_npcs(buf, &mut zone, game_type)?;
            Self::read_actions(buf, &mut zone)?;
        }

        Ok(())
    }

    fn read_hotspots(buf: &mut io::Read, zone: &mut Zone, game_type: GameType) -> io::Result<()> {
        let count = Self::read_int(buf)?;
        if count < 0 {
            return Ok(());
        }

        if game_type == GameType::Yoda {
            let mut hotspots: Vec<Hotspot> = Vec::with_capacity(count as usize);
            for _i in 0..count as usize {
                hotspots.push(Self::read_hotspot(buf)?);
            }
            zone.hotspots = hotspots;
        } else {
            for i in 0..count as usize {
                let mut hotspot = &mut zone.hotspots[i];
                Self::read_hotspot_indy(buf, &mut hotspot)?;
            }
        }
        Ok(())
    }

    fn read_npcs(buf: &mut io::Read, zone: &mut Zone, game_type: GameType) -> io::Result<()> {
        let npc_count = Self::read_int(buf)?;
        if npc_count < 0 {
            return Ok(());
        }

        if game_type == GameType::Indy {
            zone.npcs = vec![NPC {}; npc_count as usize];
        }

        assert!(
            npc_count as usize == zone.npcs.len(),
            "Number of npcs can't be changed from {} to {}!",
            zone.npcs.len(),
            npc_count
        );

        for _ in 0..npc_count {
            Self::read_npc(buf)?;
        }

        Ok(())
    }

    fn read_actions(buf: &mut io::Read, zone: &mut Zone) -> io::Result<()> {
        let action_count = Self::read_int(buf)?;
        if action_count < 0 {
            return Ok(());
        }

        assert!(
            action_count as usize == zone.actions.len(),
            "Number of actions can't be changed from {} to {}!",
            zone.actions.len(),
            action_count
        );

        for i in 0..action_count as usize {
            zone.actions[i].enabled = Self::read_bool(buf)?;
        }

        Ok(())
    }

    fn read_hotspot_indy(buf: &mut io::Read, hotspot: &mut Hotspot) -> io::Result<()> {
        hotspot.enabled = buf.read_u16::<LE>()? != 0;
        hotspot.argument = buf.read_i16::<LE>()?;

        Ok(())
    }

    fn read_hotspot(buf: &mut io::Read) -> io::Result<Hotspot> {
        let enabled = buf.read_u16::<LE>()? != 0;
        let argument = buf.read_i16::<LE>()?;
        let hotspot_type = HotspotType::from(buf.read_u32::<LE>()?);
        let x = buf.read_i16::<LE>()?;
        let y = buf.read_i16::<LE>()?;

        Ok(Hotspot {
            enabled: enabled,
            argument: argument,
            x: x,
            y: y,
            hotspot_type: hotspot_type,
        })
    }

    fn read_npc(buf: &mut io::Read) -> io::Result<()>;
}
