use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Result};

pub trait ReadHotspotExt: io::Read {
    fn read_hotspot(&mut self) -> Result<()> {
        let hotspot_type = self.read_u32::<LittleEndian>().unwrap();

        let x = self.read_u16::<LittleEndian>().unwrap();
        let y = self.read_u16::<LittleEndian>().unwrap();
        let enabled = self.read_u16::<LittleEndian>().unwrap();
        let argument = self.read_i16::<LittleEndian>().unwrap();

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadHotspotExt for R {}
