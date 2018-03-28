use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io;
use std::io::Read;

pub type Zones = Vec<Zone>;

impl super::Category for Zones {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let count = reader.read_u16::<LittleEndian>().unwrap();

        for n in 0..count {
            println!("Zone {}", n);
            (Zone {}).read_from(reader).expect("Unable to parse zone!");
        }

        Ok(())
    }
}

pub struct Zone {}
impl Zone {
    fn read_hotspot(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let hotspot_type = reader.read_u32::<LittleEndian>().unwrap();

        let x = reader.read_u16::<LittleEndian>().unwrap();
        let y = reader.read_u16::<LittleEndian>().unwrap();
        let enabled = reader.read_u16::<LittleEndian>().unwrap();
        let argument = reader.read_i16::<LittleEndian>().unwrap();

        Ok(())
    }

    fn read_action(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IACT",
            "Expected to find IACT category, found {} instead",
            marker
        );
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let condition_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..condition_count {
            self.read_action_item(reader);
        }

        let instruction_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..instruction_count {
            self.read_action_item(reader);
        }

        Ok(())
    }

    fn read_action_item(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let opcode = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..5 {
            reader.read_i16::<LittleEndian>().unwrap();
        }
        let text_length = reader.read_u16::<LittleEndian>().unwrap();
        if text_length != 0 {
            let mut text = String::new();
            reader.take(text_length.into()).read_to_string(&mut text);
        }

        Ok(())
    }

    fn read_izax(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZAX",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let unknown_count = reader.read_u16::<LittleEndian>().unwrap();

        let npc_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..npc_count {
            self.read_npc(reader);
        }
        let required_item_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..required_item_count {
            let item_id = reader.read_u16::<LittleEndian>().unwrap();
        }
        let goal_item_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..goal_item_count {
            let item_id = reader.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx2(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX2",
            "Expected to find IZX2 category, found {} instead",
            marker
        );
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let provided_item_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..provided_item_count {
            let item_id = reader.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx3(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX3",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let puzzle_npc_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..puzzle_npc_count {
            let npc_id = reader.read_u16::<LittleEndian>().unwrap();
        }

        Ok(())
    }

    fn read_izx4(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZX4",
            "Expected to find IZAX category, found {} instead",
            marker
        );
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let unknown = reader.read_u16::<LittleEndian>().unwrap();

        Ok(())
    }
    fn read_npc(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let character = reader.read_u16::<LittleEndian>().unwrap();
        let x = reader.read_u16::<LittleEndian>().unwrap();
        let y = reader.read_u16::<LittleEndian>().unwrap();
        let unknown1 = reader.read_i16::<LittleEndian>().unwrap();
        let unknown2 = reader.read_i32::<LittleEndian>().unwrap();
        for _ in 0..0x20 {
            reader.read_i8().unwrap();
        }

        Ok(())
    }
}

impl super::Category for Zone {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let planet = reader.read_u16::<LittleEndian>().unwrap();
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let index = reader.read_u16::<LittleEndian>().unwrap();
        let mut marker = String::new();
        reader.take(4).read_to_string(&mut marker);
        assert!(
            marker == "IZON",
            "Expected to find IZON category, found {} instead",
            marker
        );
        let size2 = reader.read_u32::<LittleEndian>().unwrap();
        let width = reader.read_u16::<LittleEndian>().unwrap();
        let height = reader.read_u16::<LittleEndian>().unwrap();
        let ztype = reader.read_u32::<LittleEndian>().unwrap();
        let padding = reader.read_u16::<LittleEndian>().unwrap();
        let planet_again = reader.read_u16::<LittleEndian>().unwrap();
        assert!(
            planet == planet_again,
            "Expected to find the same planet again"
        );
        let mut tile_ids = Vec::new();
        reader
            .take((3 * width * height * 2).into())
            .read_to_end(&mut tile_ids);

        let hotspot_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..hotspot_count {
            self.read_hotspot(reader);
        }

        self.read_izax(reader);
        self.read_izx2(reader);
        self.read_izx3(reader);
        self.read_izx4(reader);

        let action_count = reader.read_u16::<LittleEndian>().unwrap();
        for _ in 0..action_count {
            self.read_action(reader);
        }

        Ok(())
    }
}
