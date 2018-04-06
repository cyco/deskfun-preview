use my_byte_order::ByteOrderExt;
use std::io;

pub trait ReadSetupImageExt: io::Read {
    fn read_setup_image(&mut self) -> io::Result<()> {
        let size = self.read_u32_le()? as usize;
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSetupImageExt for R {}
