use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use byteorder::{ReadBytesExt, LE};
use std::io;

pub trait ReadSoundExt: io::Read {
    fn read_sounds(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()?;
        let count = -self.read_i16::<LE>()?;

        for _ in 0..count {
            let size = self.read_u16::<LE>()? as usize;
            let mut buffer = vec![0; size];
            self.read_exact(&mut buffer)?;
            let text = ISO_8859_1.decode(&buffer, DecoderTrap::Strict);
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadSoundExt for R {}
