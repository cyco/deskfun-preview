use byteorder::ReadBytesExt;
use std;
use std::io;

use super::save_game::SaveGame;
use game_data::zone::Zone;

use super::super::game_type::*;

use super::indy::Indy;
use super::yoda::Yoda;
use super::SaveGameReading;

type SaveGameReader = fn(&mut dyn io::Read, &mut Vec<Zone>) -> io::Result<SaveGame>;

pub trait ReadSaveGameExt: ReadBytesExt + std::marker::Sized {
    fn read_save_game_type(&mut self) -> io::Result<GameType> {
        let mut buf = vec![0; 9];
        self.read_exact(&mut buf)?;

        match String::from_utf8(buf) {
            Ok(string) => match string.as_str() {
                Yoda::MAGIC => Ok(GameType::Yoda),
                Indy::MAGIC => Ok(GameType::Indy),
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Valid file header was not found 1.",
                )),
            },
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Valid file header was not found 2.",
            )),
        }
    }

    fn read_save_game(&mut self) -> io::Result<(GameType, SaveGameReader)> {
        match self.read_save_game_type()? {
            GameType::Indy => Ok((GameType::Indy, Indy::read_save_game)),
            GameType::Yoda => Ok((GameType::Yoda, Yoda::read_save_game)),
        }
    }
}

impl<R: io::Read + std::marker::Sized + io::Seek> ReadSaveGameExt for R {}
