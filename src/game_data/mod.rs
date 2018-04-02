use std::io::{self, Read, Result};

pub mod action;
pub mod character;
pub mod end;
pub mod hotspot;
pub mod item;
pub mod npc;
pub mod puzzle;
pub mod setup_image;
pub mod sounds;
pub mod tile;
pub mod version;
pub mod zone;

use self::character::ReadCharactersExt;
use self::end::ReadEndExt;
use self::item::ReadItemsExt;
use self::puzzle::ReadPuzzlesExt;
use self::setup_image::ReadSetupImageExt;
use self::sounds::ReadSoundExt;
use self::tile::{ReadTileExt, Tile};
use self::version::ReadVersionExt;
use self::zone::*;

pub struct GameData {
    pub zones: Vec<Zone>,
    pub tiles: Vec<Tile>,
}

pub trait ReadGameDataExt: io::Read {
    fn read_game_data(&mut self) -> Result<GameData> {
        let mut zones = Vec::new();
        let mut tiles = Vec::new();

        loop {
            let mut category_name = String::new();
            self.take(4)
                .read_to_string(&mut category_name)
                .expect("Unable to read category name");

            match category_name.as_ref() {
                "VERS" => self.read_version(),
                "STUP" => self.read_setup_image(),
                "SNDS" => self.read_sounds(),
                "TILE" => {
                    tiles = self.read_tiles()?;
                    Ok(())
                }
                "ZONE" => {
                    zones = self.read_zones()?;
                    Ok(())
                }
                "PUZ2" => self.read_puzzles(),
                "CHAR" => self.read_characters(),
                "CHWP" => self.read_character_weapons(),
                "CAUX" => self.read_character_auxiliaries(),
                "TNAM" => self.read_tile_names(),
                "ENDF" => {
                    self.read_end()?;
                    break;
                }
                _ => panic!("Unknown category {} encountered", category_name),
            }?;
        }

        Ok(GameData { zones: zones, tiles: tiles })
    }
}

impl<R: io::Read + ?Sized> ReadGameDataExt for R {}
