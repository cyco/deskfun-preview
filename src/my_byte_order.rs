use byteorder::{LittleEndian, ReadBytesExt};
use std::io;

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
}

impl<R: ReadBytesExt + ?Sized> ByteOrderExt for R {}
