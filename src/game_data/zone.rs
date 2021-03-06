use super::super::GameType;
use super::marker::ReadMarkerExt;
use byteorder::{ReadBytesExt, LE};
use std::io;

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
    fn read_zones(&mut self, game_type: GameType) -> io::Result<Vec<Zone>> {
        let mut count = self.read_u16::<LE>()?;
        let mut zones = Vec::with_capacity(count as usize);

        if game_type == GameType::Indy {
            let _unknown = self.read_u16::<LE>()?;
            count = self.read_u16::<LE>()?;
        }

        for i in 0..count {
            zones.push(self.read_zone(i, game_type)?);
        }

        Ok(zones)
    }

    fn read_zone(&mut self, mut index: u16, game_type: GameType) -> io::Result<Zone> {
        let mut planet: u16 = 0;
        if game_type == GameType::Yoda {
            planet = self.read_u16::<LE>()?;
            let _size = self.read_u32::<LE>()?;
            index = self.read_u16::<LE>()?;
        }

        self.read_category_marker("IZON")?;
        let _size2 = self.read_u32::<LE>()?;
        let width = self.read_u16::<LE>()?;
        let height = self.read_u16::<LE>()?;
        let _ztype = self.read_u32::<LE>()?;
        if game_type == GameType::Yoda {
            let _padding = self.read_u16::<LE>()?;
            let planet_again = self.read_u16::<LE>()?;
            assert!(
                planet == planet_again,
                "Expected to find the same planet again"
            );
        }

        let mut tile_ids = vec![0; 3 * width as usize * height as usize];
        self.read_i16_into::<LE>(&mut tile_ids)?;

        if game_type == GameType::Indy {
            return Ok(Zone {
                id: index as usize,
                width,
                height,
                hotspots: Vec::new(),
                npcs: Vec::new(),
                actions: Vec::new(),
                tiles: tile_ids,
            });
        }

        let hotspot_count = self.read_u16::<LE>()?;
        let mut hotspots = Vec::with_capacity(hotspot_count as usize);
        for _ in 0..hotspot_count {
            hotspots.push(self.read_hotspot()?);
        }

        let (npcs, _) = self.read_izax()?;
        self.read_izx2()?;
        self.read_izx3()?;
        self.read_izx4()?;

        let action_count = self.read_u16::<LE>()?;
        let mut actions = Vec::with_capacity(action_count.into());
        for _ in 0..action_count {
            actions.push(self.read_action()?);
        }

        Ok(Zone {
            id: index as usize,
            width,
            height,
            hotspots,
            npcs,
            actions,
            tiles: tile_ids,
        })
    }

    fn read_izax(&mut self) -> io::Result<(Vec<NPC>, ())> {
        self.read_category_marker("IZAX")?;

        let _size = self.read_u32::<LE>()?;
        let _unknown_count = self.read_u16::<LE>()?;

        let npc_count = self.read_u16::<LE>()?;
        let mut npcs = Vec::with_capacity(npc_count.into());
        for _ in 0..npc_count {
            npcs.push(self.read_npc()?);
        }

        let required_item_count = self.read_u16::<LE>()?;
        let mut required_item_ids = vec![0 as u16; required_item_count as usize];
        self.read_u16_into::<LE>(&mut required_item_ids)?;

        let goal_item_count = self.read_u16::<LE>()?;
        let mut goal_item_ids = vec![0 as u16; goal_item_count as usize];
        self.read_u16_into::<LE>(&mut goal_item_ids)?;

        Ok((npcs, ()))
    }

    fn read_izx2(&mut self) -> io::Result<()> {
        self.read_category_marker("IZX2")?;

        let _size = self.read_u32::<LE>()?;
        let provided_item_count = self.read_u16::<LE>()?;
        let mut provided_item_ids = vec![0 as u16; provided_item_count as usize];
        self.read_u16_into::<LE>(&mut provided_item_ids)?;

        Ok(())
    }

    fn read_izx3(&mut self) -> io::Result<()> {
        self.read_category_marker("IZX3")?;

        let _size = self.read_u32::<LE>()?;
        let puzzle_npc_count = self.read_u16::<LE>()?;
        let mut puzzle_npc_ids = vec![0 as u16; puzzle_npc_count as usize];
        self.read_u16_into::<LE>(&mut puzzle_npc_ids)?;

        Ok(())
    }

    fn read_izx4(&mut self) -> io::Result<()> {
        self.read_category_marker("IZX4")?;

        let _size = self.read_u32::<LE>()?;
        let _unknown = self.read_u16::<LE>()?;

        Ok(())
    }

    fn read_zaux(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }

    fn read_zax2(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }

    fn read_zax3(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }

    fn read_zax4(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }

    fn read_zone_names(&mut self, _zones: &mut Vec<Zone>) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadZoneExt for R {}
