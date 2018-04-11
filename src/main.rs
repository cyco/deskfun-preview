extern crate byteorder;
extern crate elapsed;
extern crate encoding;
extern crate image;

mod game_data;
mod game_type;
mod io;
mod palette;
mod point;
mod save_game;
mod tile_renderer;
mod zone_renderer;

use image::png::PNGEncoder;
use std::fs;
use std::io::prelude::*;
use std::path;

use game_data::ReadGameDataExt;
use game_type::GameType;
use palette::Palette;
use save_game::ReadSaveGameExt;
use zone_renderer::render_zone;

const SAVE_FILE_EXTENSION: &str = "wld";

pub fn main() {
    let save_games_paths = std::env::args()
        .filter(|arg| arg.ends_with(SAVE_FILE_EXTENSION))
        .map(|path| path::PathBuf::from(path));

    let mut yoda_data = {
        let path = path::Path::new("./assets/yoda.data");
        let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
        buffer
            .read_game_data(GameType::Yoda)
            .expect("Unable to read game data file!")
    };

    let mut indy_data = {
        let path = path::Path::new("./assets/indy.data");
        let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
        buffer
            .read_game_data(GameType::Indy)
            .expect("Unable to read game data file!")
    };

    let mut indy_palette = {
        let path = path::Path::new("./assets/indy.pal");
        let mut buffer = fs::File::open(&path).expect("Unable to open palette file!");
        Palette::new(&mut buffer).expect("Unable to read palette!")
    };

    let mut yoda_palette = {
        let path = path::Path::new("./assets/yoda.pal");
        let mut buffer = fs::File::open(&path).expect("Unable to open palette file!");
        Palette::new(&mut buffer).expect("Unable to read palette!")
    };

    for save_game_path in save_games_paths {
        println!("Reading {:?}", save_game_path);
        let mut buffer =
            fs::File::open(save_game_path.as_path()).expect("Unable to open save game");
        let (game_type, reader) = buffer.read_save_game().expect("Unable to read save game");
        let game_data = match game_type {
            GameType::Indy => &mut indy_data,
            GameType::Yoda => &mut yoda_data,
        };
        let palette = match game_type {
            GameType::Indy => &mut indy_palette,
            GameType::Yoda => &mut yoda_palette,
        };

        match reader(&mut buffer, &mut game_data.zones) {
            Err(why) => println!("error: {:?}", why),
            Ok(game) => {
                let mut buffer = Vec::new();
                let result = render_zone(
                    game_data,
                    palette,
                    game.current_zone_id,
                    game.position_on_zone,
                );
                {
                    let encoder = PNGEncoder::new(&mut buffer);
                    encoder
                        .encode(&result, 288, 288, image::ColorType::RGBA(8))
                        .expect("Unable to write output file!");
                }
                buffer.shrink_to_fit();

                let mut output_path = path::PathBuf::from(save_game_path);
                output_path.set_extension("png");
                println!("Writing {}", output_path.display());
                let mut out_file =
                    fs::File::create(output_path).expect("Unable to open output file!");
                out_file
                    .write(&buffer)
                    .expect("Unable to write output file!");
            }
        }
    }
}
