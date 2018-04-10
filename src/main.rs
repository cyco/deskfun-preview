use std::error::Error;
use std::ffi::CStr;
use std::fs;
use std::io::*;
use std::os::raw::c_char;
use std::path;
use std::string;

extern crate byteorder;
extern crate encoding;
extern crate image;

mod game_type;
mod io;
mod point;

use game_type::*;

mod palette;
use palette::*;

mod game_data;
use game_data::ReadGameDataExt;

mod tile_renderer;

mod zone_renderer;
use zone_renderer::ZoneRenderer;

use image::png::PNGEncoder;

mod save_game;
use save_game::ReadSaveGameExt;

/****/
// use self::palette;
use self::tile_renderer::TileRenderer;
// use image;
// use image::png::PNGEncoder;

// use std;
// use std::fs;
use std::io::prelude::*;
/****/

extern crate elapsed;
use elapsed::measure_time;

const SAVE_FILE_EXTENSION: &str = "wld";

pub fn main() {
    let save_games_paths = std::env::args()
        .filter(|arg| arg.ends_with(SAVE_FILE_EXTENSION))
        .map(|path| path::PathBuf::from(path));

    let mut yoda_data = {
        let path =
            path::Path::new("/Users/chris/Shared/Development/webfun-preview/assets/yoda.data");
        let (elapsed, mut game_data) = measure_time(|| {
            let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
            buffer
                .read_game_data(GameType::Yoda)
                .expect("Unable to read game data file!")
        });
        println!("reading game file: {}", elapsed);
        game_data
    };

    let mut indy_data = {
        let path =
            path::Path::new("/Users/chris/Shared/Development/webfun-preview/assets/indy.data");
        let (elapsed, mut game_data) = measure_time(|| {
            let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
            buffer
                .read_game_data(GameType::Indy)
                .expect("Unable to read game data file!")
        });
        println!("reading game file: {}", elapsed);
        game_data
    };

    /* Dump tiles
    {
        let mut file = std::fs::File::open(&std::path::Path::new(
            "/Users/chris/Shared/Development/webfun-preview/assets/yoda.pal",
        )).expect("");
        let count = indy_data.tiles.len();
        let palette = palette::Palette::new(&mut file).expect("");
        let renderer = TileRenderer::new(yoda_data.tiles, palette);
        for i in 0..count {
            let mut buffer = vec![0; 32 * 32 * 4];
            let mut result = Vec::new();
            renderer.render(i as u16, 0, 0, 32, &mut buffer);
            {
                let encoder = PNGEncoder::new(&mut result);
                encoder
                    .encode(&buffer, 32, 32, image::ColorType::RGBA(8))
                    .expect("Unable to write output file");
            }
            result.shrink_to_fit();
            let path_str = format!("/Users/chris/Desktop/yoda_tiles/{}.png", i);
            let path = std::path::PathBuf::from(path_str);
            std::fs::File::create(&path.as_path())
                .expect("")
                .write(&result)
                .expect("");
        }
    }
 // */

    for save_game_path in save_games_paths {
        println!("Reading {:?}", save_game_path);
        let (elapsed, save_game) = measure_time(|| {
            /*
            let mut buffer = fs::File::open(save_game_path.as_path()).expect("");
            let game_type = buffer.read_save_game_type().expect("");
            let game_data = match game_type {
                GameType::Indy => &mut indy_data,
                GameType::Yoda => &mut yoda_data,
            };

            buffer = fs::File::open(save_game_path.as_path()).expect("");
            buffer.read_save_game(&mut game_data.zones)
            // */

            //*
            let mut buffer = fs::File::open(save_game_path.as_path())?;
            let (game_type, reader) = buffer.read_save_game()?;
            let game_data = match game_type {
                GameType::Indy => &mut indy_data,
                GameType::Yoda => &mut yoda_data,
            };
            reader(&mut buffer, &mut game_data.zones)
            // */
        });

        if let Err(why) = save_game {
            println!("error: {:?}", why);
        }

        println!("{}", elapsed);
    }
}
