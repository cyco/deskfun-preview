use my_byte_order::ByteOrderExt;
use std::io;
use std::io::Read;
use std::io::SeekFrom;
use std::vec::*;

use super::point::Point;
use game_data::hotspot::Hotspot;
use game_data::hotspot::HotspotType;
use game_data::npc::NPC;
use game_data::tile;
use game_data::zone;
use game_data::zone::Zone;

use super::game_type::*;

pub struct SaveGame {
    pub current_zone_id: u16,
    pub position_on_zone: Point,
}

pub trait ReadSaveGameExt: ByteOrderExt + io::Seek {
    fn read_save_game(&mut self, mut zones: &mut Vec<Zone>) -> io::Result<SaveGame> {
        let game_type = self.read_save_game_type()?;

        let seed = self.read_u32_le()? & 0xFFFF;
        if game_type == GameType::Yoda {
            let planet = self.read_u32_le()?;
            let on_dagobah = self.read_u32_le()? != 0;
        }

        let puzzle_id_count_1 = self.read_u16_le()?;
        let mut puzzle_ids_1 = Vec::with_capacity(puzzle_id_count_1.into());
        for _ in 0..puzzle_id_count_1 {
            puzzle_ids_1.push(self.read_i16_le()?);
        }

        if game_type == GameType::Yoda {
            let puzzle_id_count_2 = self.read_u16_le()?;
            let mut puzzle_ids_2 = Vec::with_capacity(puzzle_id_count_2.into());
            for _ in 0..puzzle_id_count_2 {
                puzzle_ids_2.push(self.read_i16_le()?);
            }

            self.read_dagobah(&mut zones, game_type)?;
        }
        self.read_world(&mut zones, game_type)?;

        let inventory_count = if game_type == GameType::Yoda {
            self.read_u32_le()? as usize
        } else {
            self.read_u16_le()? as usize
        };
        let mut inventory_item_ids = Vec::with_capacity(inventory_count as usize);
        for _ in 0..inventory_count {
            inventory_item_ids.push(self.read_i16_le()?);
        }

        let current_zone_id = self.read_u16_le()?;
        let pos_x_on_world = self.read_u32_le()?;
        let pos_y_on_world = self.read_u32_le()?;

        let mut point: Point = Point(0, 0);
        if game_type == GameType::Yoda {
            let current_weapon = self.read_i16_le()?;
            let currentAmmo = if current_weapon >= 0 {
                self.read_i16_le()?
            } else {
                -1
            };

            let force_ammo = self.read_i16_le()?;
            let blaster_ammo = self.read_i16_le()?;
            let blaster_rifle_ammo = self.read_i16_le()?;

            let pos_x_on_zone = self.read_u32_le()? as usize / tile::WIDTH;
            let pos_y_on_zone = self.read_u32_le()? as usize / tile::HEIGHT;

            let damage_taken = self.read_u32_le()?;
            let lives_left = self.read_u32_le()?;
            let difficulty = self.read_u32_le()?;
            let time_elapsed = self.read_u32_le()?;

            let world_size = self.read_i16_le()?;
            let unknown_count = self.read_i16_le()?;
            let unknown_sum = self.read_i16_le()?;
            let unknown_thing = self.read_i16_le()?;

            let goal_puzzle = self.read_u32_le()?;
            let goal_puzzle_again = self.read_u32_le()?;

            point = Point(pos_x_on_zone as i64, pos_y_on_zone as i64);
        } else {
            let u1 = self.read_i16_le()?;
            let u2 = self.read_i16_le()?;
            let u3 = self.read_i16_le()?;
            let u4 = self.read_i16_le()?;
            let u5 = self.read_i16_le()?;

            let u6 = self.read_i16_le()?;
            let u7 = self.read_i16_le()?;
            let u8 = self.read_i16_le()?;
            let u9 = self.read_i16_le()?;
        }

        assert!(
            self.seek(SeekFrom::Current(0))? == self.seek(SeekFrom::End(0))?,
            "Not at end"
        );

        Ok(SaveGame {
            current_zone_id: current_zone_id,
            position_on_zone: point,
        })
    }

    fn read_dagobah(&mut self, mut zones: &mut Vec<Zone>, game_type: GameType) -> io::Result<()> {
        for y in 4..=5 {
            for x in 4..=5 {
                self.read_world_item(x, y, game_type)?;
            }
        }

        self.read_world_details(&mut zones, game_type)?;
        Ok(())
    }

    fn read_world(&mut self, mut zones: &mut Vec<Zone>, game_type: GameType) -> io::Result<()> {
        for y in 0..10 {
            for x in 0..10 {
                self.read_world_item(x, y, game_type)?;
            }
        }

        self.read_world_details(&mut zones, game_type)?;
        Ok(())
    }

    fn read_world_item(&mut self, x: u8, y: u8, game_type: GameType) -> io::Result<()> {
        if game_type == GameType::Yoda {
            let visited = self.read_u32_le()? != 0;
            let solved_1 = self.read_u32_le()? != 0;
            let solved_2 = self.read_u32_le()? != 0;
            let solved_3 = self.read_u32_le()? != 0;
            let solved_4 = self.read_u32_le()? != 0;

            let zone_id = self.read_i16_le()?;
            let field_c = self.read_i16_le()?;
            let required_item_id = self.read_i16_le()?;
            let find_item_id = self.read_i16_le()?;
            let field_ea = self.read_i16_le()?;
            let additional_required_item_id = self.read_i16_le()?;
            let field_16 = self.read_i16_le()?;
            let npc_id = self.read_i16_le()?;

            let unknown_1 = self.read_i32_le()?;
            let unknown_2 = self.read_i16_le()?;
        } else {
            let visited = self.read_u16_le()? != 0;
            let solved_1 = self.read_u16_le()? != 0;
            let solved_2 = self.read_u16_le()? != 0;

            let zone_id = self.read_i16_le()?;
            let field_c = self.read_i16_le()?;

            let required_item_id = self.read_i16_le()?;
            let find_item_id = self.read_i16_le()?;

            let npc_id = self.read_i16_le()?;
            // possibly zone or puzzle type
            let unkonwn = self.read_i16_le()?;
        }

        Ok(())
    }

    fn read_world_details(
        &mut self,
        mut zones: &mut Vec<Zone>,
        game_type: GameType,
    ) -> io::Result<()> {
        let mut x: i16;
        let mut y: i16;
        loop {
            if game_type == GameType::Yoda {
                y = self.read_i32_le()? as i16;
                x = self.read_i32_le()? as i16;
            } else {
                y = self.read_i16_le()? as i16;
                x = self.read_i16_le()? as i16;
            }

            if x == -1 || y == -1 {
                break;
            }

            let zone_id = self.read_i16_le()?;
            let visited = if game_type == GameType::Yoda {
                self.read_u32_le()? != 0
            } else {
                self.read_u16_le()? != 0
            };

            self.read_room(zone_id, visited, &mut zones, game_type)?;
        }

        Ok(())
    }

    fn read_rooms(
        &mut self,
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

                let zone_id = self.read_i16_le()?;
                let visited = if game_type == GameType::Yoda {
                    self.read_u32_le()? != 0
                } else {
                    self.read_u16_le()? != 0
                };

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
            self.read_room(zone_id, visited, &mut zones, game_type)?;
        }

        if start + 1 < count {
            self.read_rooms(zone_id, &mut zones, start + 1, game_type)?;
        }

        Ok(())
    }

    fn read_room(
        &mut self,
        zone_id: i16,
        visited: bool,
        mut zones: &mut Vec<Zone>,
        game_type: GameType,
    ) -> io::Result<()> {
        {
            let mut zone: &mut Zone = &mut zones[zone_id as usize];
            self.read_zone(&mut zone, visited, game_type)?;
        }

        self.read_rooms(zone_id, &mut zones, 0, game_type)?;
        Ok(())
    }

    fn read_zone(&mut self, zone: &mut Zone, visited: bool, game_type: GameType) -> io::Result<()> {
        if visited {
            if game_type == GameType::Yoda {
                let counter = self.read_u32_le()?;
                let random = self.read_u32_le()?;
                let doorInX = self.read_i32_le()?;
                let doorInY = self.read_i32_le()?;
                let padding = self.read_u16_le()?;
                let planet = self.read_i16_le()?;
            } else {
                let counter = self.read_u16_le()?;
                let random = self.read_u16_le()?;
                let doorInX = self.read_i16_le()?;
                let doorInY = self.read_i16_le()?;
            }

            let mut tile_ids = Vec::new();
            for _ in 0..zone.width as u64 * zone.height as u64 * zone::LAYERS as u64 {
                tile_ids.push(self.read_i16_le());
            }
        }

        let mut __visited = visited;
        let mut hotspot_count = -1;
        if game_type == GameType::Yoda {
            __visited = { self.read_u32_le()? != 0 };
            hotspot_count = self.read_i32_le()? as i64;
        }

        if game_type == GameType::Indy {
            __visited = self.read_u16_le()? != 0;
            hotspot_count = self.read_i16_le()? as i64;
        }

        if hotspot_count as usize != zone.hotspots.len() as usize {
            println!(
                "-- change hotspots from {} to {}",
                hotspot_count,
                zone.hotspots.len()
            );
        }

        if hotspot_count > 0 {
            if game_type == GameType::Yoda {
                let mut hotspots: Vec<Hotspot> = Vec::with_capacity(hotspot_count as usize);
                for _i in 0..hotspot_count as usize {
                    hotspots.push(self.read_hotspot()?);
                }
                zone.hotspots = hotspots;
            } else {
                for i in 0..hotspot_count as usize {
                    let mut hotspot = &mut zone.hotspots[i];
                    self.read_hotspot_indy(&mut hotspot)?;
                }
            }
        }

        if visited {
            let npc_count = if game_type == GameType::Yoda {
                self.read_i32_le()? as i64
            } else {
                self.read_i16_le()? as i64
            };

            if npc_count > 0 {
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
                    if game_type == GameType::Indy {
                        self.read_npc_indy()?;
                    } else {
                        self.read_npc()?;
                    }
                }
            }
        }

        if visited {
            let action_count = if game_type == GameType::Yoda {
                self.read_i32_le()? as i64
            } else {
                self.read_i16_le()? as i64
            };

            if action_count > 0 {
                assert!(
                    action_count as usize == zone.actions.len(),
                    "Number of actions can't be changed from {} to {}!",
                    zone.actions.len(),
                    action_count
                );

                for _ in 0..action_count {
                    if game_type == GameType::Yoda {
                        let action_enabled = self.read_u32_le()? != 0;
                    } else {
                        let action_enabled = self.read_u16_le()? != 0;
                    }
                }
            }
        }

        Ok(())
    }

    fn read_hotspot_indy(&mut self, hotspot: &mut Hotspot) -> io::Result<()> {
        hotspot.enabled = self.read_u16_le()? != 0;
        hotspot.argument = self.read_i16_le()?;

        Ok(())
    }

    fn read_hotspot(&mut self) -> io::Result<Hotspot> {
        let enabled = self.read_u16_le()? != 0;
        let argument = self.read_i16_le()?;
        let hotspot_type = HotspotType::from(self.read_u32_le()?);
        let x = self.read_i16_le()?;
        let y = self.read_i16_le()?;

        Ok(Hotspot {
            enabled: enabled,
            argument: argument,
            x: x,
            y: y,
            hotspot_type: hotspot_type,
        })
    }

    fn read_npc_indy(&mut self) -> io::Result<()> {
        let mut data = vec![0; 0x20];
        self.read_exact(&mut data)
    }

    fn read_npc(&mut self) -> io::Result<()> {
        let character_id = self.read_i16_le()?;
        let x = self.read_i16_le()?;
        let y = self.read_i16_le()?;
        let field_a = self.read_i16_le()?;
        let enabled = self.read_u32_le()? != 0;
        let field_10 = self.read_i16_le()?;
        let field_x__ = self.read_i16_le()?;
        let field_y__ = self.read_i16_le()?;
        let current_frame = self.read_i16_le()?;
        let field_18 = self.read_u32_le()?;
        let field_1c = self.read_u32_le()?;
        let field_2 = self.read_u32_le()?;
        let field_x_ = self.read_i16_le()?;
        let field_y_ = self.read_i16_le()?;
        let field_3c = self.read_i16_le()?;
        let field_3e = self.read_i16_le()?;
        let field_60 = self.read_i16_le()?;
        let field_26 = self.read_i16_le()?;
        let field_2c = self.read_u32_le()?;
        let field_34 = self.read_u32_le()?;
        let field_28 = self.read_u32_le()?;

        let field_24 = self.read_i16_le()?;
        let unknown = self.read_i16_le()?;

        for _ in 0..4 {
            self.read_u32_le()?;
            self.read_u32_le()?;
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized + io::Seek> ReadSaveGameExt for R {}
