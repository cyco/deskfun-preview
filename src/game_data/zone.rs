use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Result};

use super::npc::ReadNPCExt;
use super::hotspot::ReadHotspotExt;

pub trait ReadZoneExt: io::Read {
    fn read_zones(&mut self) -> Result<()> {
        let count = self.read_u16::<LittleEndian>().unwrap();

        for n in 0..count {
            self.read_zone().expect("Unable to parse zone!");
        }

        Ok(())
    }

    fn read_zone(&mut self) -> Result<()> {
        let planet = self.read_u16::<LittleEndian>().unwrap();
        let size = self.read_u32::<LittleEndian>().unwrap();
        let index = self.read_u16::<LittleEndian>().unwrap();
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZON",
            "Expected to find IZON category, found {} instead",
            marker
        );
        let size2 = self.read_u32::<LittleEndian>().unwrap();
        let width = self.read_u16::<LittleEndian>().unwrap();
        let height = self.read_u16::<LittleEndian>().unwrap();
        let ztype = self.read_u32::<LittleEndian>().unwrap();
        let padding = self.read_u16::<LittleEndian>().unwrap();
        let planet_again = self.read_u16::<LittleEndian>().unwrap();
        assert!(
            planet == planet_again,
            "Expected to find the same planet again"
        );
        let mut tile_ids = Vec::new();
        self.take((3 * width * height * 2).into())
            .read_to_end(&mut tile_ids);

        let hotspot_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..hotspot_count {
            self.read_hotspot();
        }

        self.read_izax();
        self.read_izx2();
        self.read_izx3();
        self.read_izx4();

        let action_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..action_count {
            self.read_action();
        }

        Ok(())
    }

    fn read_action(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IACT",
            "Expected to find IACT category, found {} instead",
            marker
        );
        let size = self.read_u32::<LittleEndian>().unwrap();
        let condition_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..condition_count {
            self.read_action_item();
        }

        let instruction_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..instruction_count {
            self.read_action_item();
        }

        Ok(())
    }

    fn read_action_item(&mut self) -> Result<()> {
        let opcode = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..5 {
            self.read_i16::<LittleEndian>().unwrap();
        }
        let text_length = self.read_u16::<LittleEndian>().unwrap();
        if text_length != 0 {
            let mut text = String::new();
            self.take(text_length.into()).read_to_string(&mut text);
        }

        Ok(())
    }

    fn read_izax(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZAX",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32::<LittleEndian>().unwrap();
        let unknown_count = self.read_u16::<LittleEndian>().unwrap();

        let npc_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..npc_count {
            self.read_npc();
        }
        let required_item_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..required_item_count {
            let item_id = self.read_u16::<LittleEndian>().unwrap();
        }
        let goal_item_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..goal_item_count {
            let item_id = self.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx2(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX2",
            "Expected to find IZX2 category, found {} instead",
            marker
        );
        let size = self.read_u32::<LittleEndian>().unwrap();
        let provided_item_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..provided_item_count {
            let item_id = self.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx3(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX3",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32::<LittleEndian>().unwrap();
        let puzzle_npc_count = self.read_u16::<LittleEndian>().unwrap();
        for _ in 0..puzzle_npc_count {
            let npc_id = self.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx4(&mut self) -> Result<()> {
        let mut marker = String::new();
        self.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX4",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = self.read_u32::<LittleEndian>().unwrap();
        let unknown = self.read_u16::<LittleEndian>().unwrap();

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadZoneExt for R {}
