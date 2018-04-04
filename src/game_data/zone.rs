use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};
use std::ops::Index;

use super::action::*;
use super::hotspot::*;
use super::npc::*;

pub const LAYERS: u8 = 3;

pub struct Zone {
    pub id: usize,
    pub width: u16,
    pub height: u16,
    pub hotspots: Vec<Hotspot>,
    pub npcs: Vec<NPC>,
    pub actions: Vec<Action>,
    pub tiles: Vec<i16>,
}

impl Zone {
    pub fn tile_id_at(&self, index: (u8, u8, u8)) -> Option<i16> {
        let (x, y, z) = index;
        let tile_idx = 3 * (y as usize * self.width as usize + x as usize) + z as usize;
        match self.tiles[tile_idx] {
            -1 => None,
            tile_id => Some(tile_id),
        }
    }
}

pub trait ReadZoneExt: io::Read {
    fn read_zones(&mut self) -> Result<Vec<Zone>> {
        let count = self.read_u16_le()?;
        let mut zones = Vec::new();

        for n in 0..count {
            zones.push(self.read_zone()?);
        }

        Ok(zones)
    }

    fn read_zone(&mut self) -> Result<Zone> {
        let planet = self.read_u16_le()?;
        let size = self.read_u32_le()?;
        let index = self.read_u16_le()?;
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IZON",
            "Expected to find IZON category, found {} instead",
            marker
        );
        let size2 = self.read_u32_le()?;
        let width = self.read_u16_le()?;
        let height = self.read_u16_le()?;
        let ztype = self.read_u32_le()?;
        let padding = self.read_u16_le()?;
        let planet_again = self.read_u16_le()?;
        assert!(
            planet == planet_again,
            "Expected to find the same planet again"
        );
        let mut tile_ids = vec![0; 3 * width as usize * height as usize];
        self.take(3 * width as u64 * height as u64 * 2)
            .read_i16_le_into(&mut tile_ids)?;

        let hotspot_count = self.read_u16_le()?;
        let mut hotspots = Vec::new();
        for _ in 0..hotspot_count {
            hotspots.push(self.read_hotspot()?);
        }

        let (npcs, _) = self.read_izax()?;
        self.read_izx2();
        self.read_izx3();
        self.read_izx4();

        let action_count = self.read_u16_le()?;
        let mut actions = Vec::with_capacity(action_count.into());
        for _ in 0..action_count {
            actions.push(self.read_action()?);
        }

        Ok(Zone {
            id: index as usize,
            width: width,
            height: height,
            hotspots: hotspots,
            npcs: npcs,
            actions: actions,
            tiles: tile_ids,
        })
    }

    fn read_izax(&mut self) -> Result<(Vec<NPC>, ())> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IZAX",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32_le()?;
        let unknown_count = self.read_u16_le()?;

        let npc_count = self.read_u16_le()?;
        let mut npcs = Vec::with_capacity(npc_count.into());
        for _ in 0..npc_count {
            npcs.push(self.read_npc()?);
        }
        let required_item_count = self.read_u16_le()?;
        for _ in 0..required_item_count {
            let item_id = self.read_u16_le()?;
        }
        let goal_item_count = self.read_u16_le()?;
        for _ in 0..goal_item_count {
            let item_id = self.read_u16_le()?;
        }

        Ok((npcs, ()))
    }

    fn read_izx2(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IZX2",
            "Expected to find IZX2 category, found {} instead",
            marker
        );
        let size = self.read_u32_le()?;
        let provided_item_count = self.read_u16_le()?;
        for _ in 0..provided_item_count {
            let item_id = self.read_u16_le()?;
        }

        Ok(())
    }

    fn read_izx3(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IZX3",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32_le()?;
        let puzzle_npc_count = self.read_u16_le()?;
        for _ in 0..puzzle_npc_count {
            let npc_id = self.read_u16_le()?;
        }

        Ok(())
    }

    fn read_izx4(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IZX4",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32_le()?;
        let unknown = self.read_u16_le()?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadZoneExt for R {}
