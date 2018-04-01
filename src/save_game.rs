use my_byte_order::ByteOrderExt;
use std::io;
use std::io::{Read, Result};
use std::vec::*;

use game_data::hotspot::Hotspot;
use game_data::hotspot::HotspotType;
use game_data::zone;
use game_data::zone::Zone;

pub trait ReadSaveGameExt: ByteOrderExt {
    fn read_save_game(&mut self, mut zones: &mut Vec<Zone>) -> Result<()> {
        let mut file_magic = String::new();
        self.take(9).read_to_string(&mut file_magic)?;
        assert!(
            file_magic == "YODASAV44",
            "File header {} does not match expected value",
            file_magic
        );

        let seed = self.read_u32_le()? & 0xFFFF;
        let planet = self.read_u32_le()?;

        let on_dagobah = self.read_u32_le()? != 0;

        let puzzle_id_count_1 = self.read_u16_le()?;
        let mut puzzle_ids_1 = Vec::with_capacity(puzzle_id_count_1.into());
        for _ in 0..puzzle_id_count_1 {
            puzzle_ids_1.push(self.read_i16_le()?);
        }

        let puzzle_id_count_2 = self.read_u16_le()?;
        let mut puzzle_ids_2 = Vec::with_capacity(puzzle_id_count_2.into());
        for _ in 0..puzzle_id_count_2 {
            puzzle_ids_2.push(self.read_i16_le()?);
        }

        self.read_dagobah(&mut zones);
        self.read_world(&mut zones);

        let inventory_count = self.read_u32_le()?;
        let mut inventory_item_ids = Vec::with_capacity(inventory_count as usize);
        for _ in 0..inventory_count {
            inventory_item_ids.push(self.read_i16_le()?);
        }

        let current_zone_id = self.read_u16_le()?;
        let pos_x_on_world = self.read_u32_le()?;
        let pos_y_on_world = self.read_u32_le()?;

        let current_weapon = self.read_i16_le()?;
        let currentAmmo = if current_weapon >= 0 {
            self.read_i16_le()?
        } else {
            -1
        };

        let force_ammo = self.read_i16_le()?;
        let blaster_ammo = self.read_i16_le()?;
        let blaster_rifle_ammo = self.read_i16_le()?;

        let pos_x_on_zone = self.read_u32_le()? as f64 / 32.0;
        let pos_y_on_zone = self.read_u32_le()? as f64 / 32.0;

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

        Ok(())
    }

    fn read_dagobah(&mut self, mut zones: &mut Vec<Zone>) -> Result<()> {
        for y in 4..=5 {
            for x in 4..=5 {
                self.read_world_item(x, y);
            }
        }

        self.read_world_details(&mut zones);
        Ok(())
    }

    fn read_world(&mut self, mut zones: &mut Vec<Zone>) -> Result<()> {
        for y in 0..10 {
            for x in 0..10 {
                self.read_world_item(x, y);
            }
        }

        self.read_world_details(&mut zones);
        Ok(())
    }

    fn read_world_item(&mut self, x: u8, y: u8) -> Result<()> {
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

        Ok(())
    }

    fn read_world_details(&mut self, mut zones: &mut Vec<Zone>) -> Result<()> {
        let mut x: i32;
        let mut y: i32;
        loop {
            x = self.read_i32_le()?;
            y = self.read_i32_le()?;

            if x == -1 || y == -1 {
                break;
            }

            let zone_id = self.read_i16_le()?;
            let visited = self.read_u32_le()? != 0;

            self.read_room(zone_id, visited, &mut zones);
        }

        println!("read_world_details_done");
        Ok(())
    }

    fn read_rooms(
        &mut self,
        zone_id: i16,
        mut zones: &mut Vec<Zone>,
        mut start: usize,
    ) -> Result<()> {
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
                let visited = self.read_u32_le()? != 0;
                assert!(
                    zone_id == door,
                    "Expected door to lead to zone {} instead of {}",
                    zone_id,
                    door
                );
                zone_ids.push((zone_id, visited));
                break;
            }
        }

        for (zone_id, visited) in zone_ids {
            self.read_room(zone_id, visited, &mut zones);
        }

        if (start + 1 < count) {
            self.read_rooms(zone_id, &mut zones, start + 1);
        }

        Ok(())
    }

    fn read_room(&mut self, zone_id: i16, visited: bool, mut zones: &mut Vec<Zone>) -> Result<()> {
        {
            let mut zone: &mut Zone = &mut zones[zone_id as usize];
            self.read_zone(&mut zone, visited);
        }

        self.read_rooms(zone_id, &mut zones, 0);

        Ok(())
    }

    fn read_zone(&mut self, zone: &mut Zone, visited: bool) -> Result<()> {
        println!("_readZone start");
        if visited {
            let counter = self.read_u32_le()?;
            let random = self.read_u32_le()?;
            let doorInX = self.read_i32_le()?;
            let doorInY = self.read_i32_le()?;
            let padding = self.read_u16_le()?;
            let planet = self.read_i16_le()?;

            let mut tile_ids = Vec::new();
            for _ in 0..zone.width as u64 * zone.height as u64 * zone::LAYERS as u64 {
                tile_ids.push(self.read_i16_le());
            }
        }

        let __visited = self.read_u32_le()? != 0;
        let hotspot_count = self.read_i32_le()?;

        /*
        assert!(
            hotspot_count as usize == zone.hotspots.len(),
            "Number of hotspots can't be changed from {} to {}!",
            zone.hotspots.len(),
            hotspot_count
        );
*/
        if hotspot_count > 0 {
            if hotspot_count as usize != zone.hotspots.len() as usize {
                println!(
                    "-- change hotspots from {} to {}",
                    hotspot_count,
                    zone.hotspots.len()
                );
            }

            let mut hotspots: Vec<Hotspot> = Vec::with_capacity(hotspot_count as usize);
            for i in 0..hotspot_count as usize {
                hotspots.push(self.read_hotspot()?);
            }
            zone.hotspots = hotspots;
        }

        if visited {
            let npc_count = self.read_i32_le()?;
            assert!(
                npc_count as usize == zone.npcs.len(),
                "Number of hotspots can't be changed from {} to {}!",
                zone.npcs.len(),
                npc_count
            );

            if npc_count > 0 {
                for _ in 0..npc_count {
                    self.read_npc();
                }
            }

            let action_count = self.read_i32_le()?;
            assert!(
                action_count as usize == zone.actions.len(),
                "Number of hotspots can't be changed from {} to {}!",
                zone.actions.len(),
                action_count
            );

            if action_count > 0 {
                for _ in 0..action_count {
                    let action_enabled = self.read_u32_le()? != 0;
                }
            }
        }

        println!("_readZone end");
        Ok(())
    }

    fn read_hotspot(&mut self) -> Result<Hotspot> {
        println!("_readHotspot start");
        let enabled = self.read_u16_le()? != 0;
        let argument = self.read_i16_le()?;
        let hotspot_type = HotspotType::from(self.read_u32_le()?);
        let x = self.read_i16_le()?;
        let y = self.read_i16_le()?;
        println!("_readHotspot end");
        Ok(Hotspot {
            enabled: enabled,
            argument: argument,
            x: x,
            y: y,
            hotspot_type: hotspot_type,
        })
    }

    fn read_npc(&mut self) -> Result<()> {
        println!("read_npc start");
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

        println!("read_npc end");
        Ok(())
    }
}

impl<R: io::Read + ?Sized + io::Seek> ReadSaveGameExt for R {}
