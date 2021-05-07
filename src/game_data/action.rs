use super::marker::ReadMarkerExt;
use super::zone::Zone;
use byteorder::{ReadBytesExt, LE};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use std::io;

pub struct ActionItem {
    pub opcode: u16,
    pub arguments: [i16; 5],
    pub text: String,
}

pub struct Action {
    pub enabled: bool,
    pub conditions: Vec<ActionItem>,
    pub instructions: Vec<ActionItem>,
}

pub trait ReadActionExt: io::Read {
    fn read_action(&mut self) -> io::Result<Action> {
        self.read_category_marker("IACT")?;

        let _size = self.read_u32::<LE>()?;
        let condition_count = self.read_u16::<LE>()? as usize;
        let mut conditions = Vec::with_capacity(condition_count);
        for _ in 0..condition_count {
            conditions.push(self.read_action_item()?);
        }

        let instruction_count = self.read_u16::<LE>()? as usize;
        let mut instructions = Vec::with_capacity(instruction_count);
        for _ in 0..instruction_count {
            instructions.push(self.read_action_item()?);
        }

        Ok(Action {
            enabled: true,
            conditions: conditions,
            instructions: instructions,
        })
    }

    fn read_action_item(&mut self) -> io::Result<ActionItem> {
        let opcode = self.read_u16::<LE>()?;
        let mut arguments = [0 as i16; 5];
        self.read_i16_into::<LE>(&mut arguments)?;

        let mut text = String::new();
        let text_length = self.read_u16::<LE>()? as usize;
        if text_length != 0 {
            let mut buffer = vec![0; text_length];
            self.read_exact(&mut buffer)?;
            text = match ISO_8859_1.decode(&buffer, DecoderTrap::Strict) {
                Ok(t) => Ok(t.to_string()),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unable to decode string.",
                )),
            }?;
        }

        Ok(ActionItem {
            opcode: opcode,
            arguments: arguments,
            text: text,
        })
    }

    fn read_actions(&mut self, zones: &mut Vec<Zone>) -> io::Result<()> {
        let _size = self.read_u32::<LE>()?;
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
