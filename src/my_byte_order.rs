use byteorder::{LittleEndian, ReadBytesExt};
use std::fs;
use std::io;
use std::io::Seek;
use std::io::*;

pub trait ByteOrderExt: ReadBytesExt + Seek {
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
}

impl<R: ReadBytesExt + ?Sized + Seek> ByteOrderExt for R {}
