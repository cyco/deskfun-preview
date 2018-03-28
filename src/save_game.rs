use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Result};

pub trait ReadSaveGameExt: io::Read {
    fn read_save_game(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSaveGameExt for R {}
