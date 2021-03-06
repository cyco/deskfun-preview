use std::io;
use std::io::Read;

pub mod action;
pub mod character;
pub mod end;
pub mod hotspot;
pub mod item;
mod marker;
pub mod npc;
pub mod puzzle;
pub mod setup_image;
pub mod sounds;
pub mod tile;
pub mod version;
pub mod zone;

use self::action::ReadActionExt;
use self::character::ReadCharactersExt;
use self::end::ReadEndExt;
use self::hotspot::ReadHotspotExt;
use self::item::ReadItemsExt;
use self::puzzle::{Puzzle, ReadPuzzlesExt};
use self::setup_image::ReadSetupImageExt;
use self::sounds::ReadSoundExt;
use self::tile::{ReadTileExt, Tile};
use self::version::ReadVersionExt;
use self::zone::*;
use super::game_type::GameType;

use elapsed::measure_time;

pub struct GameData {
    pub game_type: GameType,
    pub zones: Vec<Zone>,
    pub tiles: Vec<Tile>,
    pub puzzles: Vec<Puzzle>,
}

pub trait ReadGameDataExt: io::Read {
    fn read_game_data(&mut self, game_type: GameType) -> io::Result<GameData> {
        let mut zones = Vec::new();
        let mut tiles = Vec::new();
        let mut puzzles = Vec::new();

        loop {
            let mut category_name = String::new();
            self.take(4)
                .read_to_string(&mut category_name)
                .expect("Unable to read category name");

            let (_elapsed, result) = measure_time(|| -> io::Result<u8> {
                let _ = match category_name.as_ref() {
                    "VERS" => self.read_version(),
                    "STUP" => self.read_setup_image(),
                    "SNDS" => self.read_sounds(),
                    "TILE" => {
                        tiles = self.read_tiles()?;
                        Ok(())
                    }
                    "ZONE" => {
                        zones = self.read_zones(game_type)?;
                        Ok(())
                    }
                    "PUZ2" => {
                        puzzles = self.read_puzzles(game_type)?;
                        Ok(())
                    }
                    "CHAR" => self.read_characters(game_type),
                    "CHWP" => self.read_character_weapons(),
                    "CAUX" => self.read_character_auxiliaries(),
                    "TNAM" => self.read_tile_names(game_type),
                    "ZAUX" => self.read_zaux(),
                    "ZAX2" => self.read_zax2(),
                    "ZAX3" => self.read_zax3(),
                    "ZAX4" => self.read_zax4(),
                    "HTSP" => self.read_hotspots(&mut zones),
                    "ACTN" => self.read_actions(&mut zones),
                    "ZNAM" => self.read_zone_names(&mut zones),
                    "PNAM" => self.read_puzzle_names(),
                    "ANAM" => self.read_action_names(),
                    "ENDF" => {
                        self.read_end()?;
                        return Ok(1);
                    }
                    _ => panic!("Unknown category {} encountered", category_name),
                }?;

                Ok(0)
            });

            match result {
                Ok(1) => break,
                _ => (),
            };
        }

        Ok(GameData {
            game_type,
            zones,
            tiles,
            puzzles,
        })
    }
}

impl<R: io::Read + ?Sized> ReadGameDataExt for R {}
