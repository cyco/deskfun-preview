use super::marker::ReadMarkerExt;
use super::zone::Zone;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use byteorder::{ReadBytesExt, LE};
use std::io;

pub struct Action {}

pub trait ReadActionExt: io::Read {
    fn read_action(&mut self) -> io::Result<Action> {
        self.read_category_marker("IACT")?;

        let size = self.read_u32::<LE>()?;
        let condition_count = self.read_u16::<LE>()?;
        for _ in 0..condition_count {
            self.read_action_item()?;
        }

        let instruction_count = self.read_u16::<LE>()?;
        for _ in 0..instruction_count {
            self.read_action_item()?;
        }

        Ok(Action {})
    }

    fn read_action_item(&mut self) -> io::Result<()> {
        let opcode = self.read_u16::<LE>()?;
        let mut arguments = [0 as i16; 5];
        self.read_i16_into::<LE>(&mut arguments)?;

        let text_length = self.read_u16::<LE>()? as usize;
        if text_length != 0 {
            let mut buffer = vec![0; text_length];
            self.read_exact(&mut buffer)?;
            let text = ISO_8859_1.decode(&buffer, DecoderTrap::Strict);
        }

        Ok(())
    }

    fn read_actions(&mut self, zones: &mut Vec<Zone>) -> io::Result<()> {
        self.read_u32::<LE>()?;
        loop {
            let idx = self.read_i16::<LE>()?;
            if idx == -1 {
                break;
            }

            let count = self.read_u16::<LE>()?;

            let mut actions = Vec::with_capacity(count as usize);
            for _ in 0..count {
                actions.push(self.read_action()?);
            }
            zones[idx as usize].actions = actions;
        }

        Ok(())
    }

    fn read_action_names(&mut self) -> io::Result<()> {
        let size = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)?;

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadActionExt for R {}
