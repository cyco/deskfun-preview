use byteorder::{ReadBytesExt, LE};
use std::io;

pub trait ReadSetupImageExt: io::Read {
    fn read_setup_image(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSetupImageExt for R {}
