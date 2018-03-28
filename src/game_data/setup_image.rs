use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io;
use std::io::Read;

pub struct SetupImage {}

impl SetupImage {}

impl super::Category for SetupImage {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let mut buffer = Vec::new();
        reader.take(size.into()).read_to_end(&mut buffer);

        Ok(())
    }
}
