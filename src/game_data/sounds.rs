use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io;
use std::io::Read;

pub type Sounds = Vec<String>;

impl super::Category for Sounds {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let count = -reader.read_i16::<LittleEndian>().unwrap();

        for n in 0..count {
            let size = reader.read_u16::<LittleEndian>().unwrap();

            let mut string = String::new();
            reader
                .take(size.into())
                .read_to_string(&mut string)
                .expect("Unable to read sound name!");
            println!("Sound {} of {}:  {}", n + 1, count, string);
        }

        Ok(())
    }
}
