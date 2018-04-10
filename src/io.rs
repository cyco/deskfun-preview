use byteorder::ReadBytesExt;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use std::io;
use std::io::{ErrorKind, Read};

pub trait ByteOrderExt: ReadBytesExt {
    #[inline]
    fn read_cstring(&mut self) -> io::Result<String> {
        let mut name_buffer = Vec::new();
        loop {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }
            name_buffer.push(byte);
        }

        let mut name = String::with_capacity(name_buffer.len());

        name_buffer
            .take(name_buffer.len() as u64)
            .read_to_string(&mut name)?;

        Ok(name)
    }

    #[inline]
    fn read_cstring_with_length(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0 as u8; length];
        self.read_exact(&mut buffer)?;

        let mut name = String::with_capacity(buffer.len());

        let mut used_length = length;
        for i in 0..buffer.len() {
            if buffer[i] == 0 {
                used_length = i;
                break;
            }
        }

        buffer.take(used_length as u64).read_to_string(&mut name)?;

        Ok(name)
    }

    #[inline]
    fn read_iso_cstring_with_length(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0 as u8; length];
        self.read_exact(&mut buffer)?;

        let mut used_length = length;
        for i in 0..buffer.len() {
            if buffer[i] == 0 {
                used_length = i;
                break;
            }
        }

        match ISO_8859_1.decode(&buffer[0..used_length], DecoderTrap::Strict) {
            Err(err) => Err(io::Error::new(ErrorKind::Other, err)),
            Ok(thing) => Ok(thing),
        }
    }
}

impl<R: ReadBytesExt + ?Sized> ByteOrderExt for R {}
