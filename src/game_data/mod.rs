use std;
use std::fmt;
use std::io;
use std::io::Read;

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
use self::setup_image::SetupImage;
use self::sounds::Sounds;
use self::tile::Tiles;
use self::version::Version;
use self::zone::Zones;

pub trait Category {
    fn read_from(&mut self, reader: &mut io::Read) -> Result<(), std::string::ParseError>;
}

pub struct GameData {}

impl GameData {
    pub fn new(reader: &mut io::Read) -> Result<(GameData), std::string::ParseError> {
        loop {
            let mut category_name = String::new();
            reader
                .take(4)
                .read_to_string(&mut category_name)
                .expect("Unable to read category name");

            match category_name.as_ref() {
                "VERS" => {
                    (Version {})
                        .read_from(reader)
                        .expect("Unable to read version");
                    ()
                }
                "STUP" => {
                    (SetupImage {})
                        .read_from(reader)
                        .expect("Unable to read setup image");
                    ()
                }
                "SNDS" => {
                    Sounds::new()
                        .read_from(reader)
                        .expect("Unable to read sounds");
                    ()
                }
                "TILE" => {
                    Tiles::new()
                        .read_from(reader)
                        .expect("Unable to read sounds");
                    ()
                }
                "ZONE" => {
                    Zones::new()
                        .read_from(reader)
                        .expect("Unable to read sounds");
                    ()
                }
                "PUZ2" => {
                    reader.read_puzzles();
                }
                "CHAR" => {
                    reader.read_characters();
                }
                "CHWP" => {
                    reader.read_character_weapons();
                }
                "CAUX" => {
                    reader.read_character_auxiliaries();
                }
                "TNAM" => {
                    reader.read_tile_names();
                }
                "ENDF" => {
                    reader.read_end();
                    break;
                }
                _ => panic!("Unknown category {} encountered", category_name),
            };
        }

        Ok(GameData {})
    }
}

impl fmt::Display for GameData {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}
