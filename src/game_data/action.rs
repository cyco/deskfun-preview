use my_byte_order::ByteOrderExt;
use std::io::{self, Read, Result};

pub struct Action {}

pub trait ReadActionExt: io::Read {
    fn read_action(&mut self) -> Result<Action> {
        let mut marker = String::with_capacity(4);
        self.take(4).read_to_string(&mut marker)?;
        assert!(
            marker == "IACT",
            "Expected to find IACT category, found {} instead",
            marker
        );
        let size = self.read_u32_le()?;
        let condition_count = self.read_u16_le()?;
        for _ in 0..condition_count {
            self.read_action_item();
        }

        let instruction_count = self.read_u16_le()?;
        for _ in 0..instruction_count {
            self.read_action_item();
        }

        Ok(Action {})
    }

    fn read_action_item(&mut self) -> Result<()> {
        let opcode = self.read_u16_le()?;
        let mut arguments = [0 as i16; 5];
        self.read_i16_le_into(&mut arguments)?;

        let text_length = self.read_u16_le()?;
        if text_length != 0 {
            let mut text = String::with_capacity(text_length as usize);
            self.take(text_length.into()).read_to_string(&mut text)?;
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadActionExt for R {}
