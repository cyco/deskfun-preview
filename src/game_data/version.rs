use byteorder::{LittleEndian, ReadBytesExt};
use std;
use std::io;

pub struct Version {}

impl Version {}

impl super::Category for Version {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError> {
        reader.read_u32::<LittleEndian>();

        Ok(())
    }
}
