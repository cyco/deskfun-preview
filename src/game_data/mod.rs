use std::io::{self, Read, Result};

mod character;
mod end;
mod item;
mod puzzle;
mod setup_image;
mod sounds;
mod tile;
mod version;
mod zone;

use self::character::ReadCharactersExt;
use self::end::ReadEndExt;
use self::item::ReadItemsExt;
use self::puzzle::ReadPuzzlesExt;
use self::setup_image::ReadSetupImageExt;
use self::sounds::ReadSoundExt;
use self::tile::ReadTileExt;
use self::version::ReadVersionExt;
use self::zone::ReadZoneExt;

pub trait ReadGameDataExt: io::Read {
    fn read_game_data(&mut self) -> Result<()> {
        loop {
            let mut category_name = String::new();
            self.take(4)
                .read_to_string(&mut category_name)
                .expect("Unable to read category name");

            match category_name.as_ref() {
                "VERS" => self.read_version(),
                "STUP" => self.read_setup_image(),
                "SNDS" => self.read_sounds(),
                "TILE" => self.read_tiles(),
                "ZONE" => self.read_zones(),
                "PUZ2" => self.read_puzzles(),
                "CHAR" => self.read_characters(),
                "CHWP" => self.read_character_weapons(),
                "CAUX" => self.read_character_auxiliaries(),
                "TNAM" => self.read_tile_names(),
                "ENDF" => {
                    self.read_end();
                    break;
                }
                _ => panic!("Unknown category {} encountered", category_name),
            };
        }

        Ok(())
    }
}

impl<R: io::Read + ?Sized> ReadGameDataExt for R {}
