use byteorder::{LittleEndian, ReadBytesExt};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use std::io;
use std::io::{Read, ErrorKind};
use std::iter::*;
use std::result;

pub trait ByteOrderExt: ReadBytesExt {
    #[inline]
    fn read_i8_le(&mut self) -> io::Result<i8> {
        self.read_i8()
    }

    #[inline]
    fn read_u8_le(&mut self) -> io::Result<u8> {
        self.read_u8()
    }

    #[inline]
    fn read_i16_le(&mut self) -> io::Result<i16> {
        self.read_i16::<LittleEndian>()
    }

    #[inline]
    fn read_i32_le(&mut self) -> io::Result<i32> {
        self.read_i32::<LittleEndian>()
    }

    #[inline]
    fn read_u16_le(&mut self) -> io::Result<u16> {
        self.read_u16::<LittleEndian>()
    }

    #[inline]
    fn read_u32_le(&mut self) -> io::Result<u32> {
        self.read_u32::<LittleEndian>()
    }

    #[inline]
    fn read_i16_le_into(&mut self, mut dst: &mut [i16]) -> io::Result<()> {
        self.read_i16_into::<LittleEndian>(&mut dst)
    }

    #[inline]
    fn read_u16_le_into(&mut self, mut dst: &mut [u16]) -> io::Result<()> {
        self.read_u16_into::<LittleEndian>(&mut dst)
    }

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
            Ok(thing) => Ok(thing)
        }
    }
}

impl<R: ReadBytesExt + ?Sized> ByteOrderExt for R {}
