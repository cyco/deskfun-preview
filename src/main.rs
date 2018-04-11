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

use elapsed::measure_time;
use std::fs;
use std::path;

use game_data::ReadGameDataExt;
use game_type::GameType;
use save_game::ReadSaveGameExt;

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

    for save_game_path in save_games_paths {
        println!("Reading {:?}", save_game_path);
        let (elapsed, save_game) = measure_time(|| {
            let mut buffer = fs::File::open(save_game_path.as_path())?;
            let (game_type, reader) = buffer.read_save_game()?;
            let game_data = match game_type {
                GameType::Indy => &mut indy_data,
                GameType::Yoda => &mut yoda_data,
            };
            reader(&mut buffer, &mut game_data.zones)
        });

        if let Err(why) = save_game {
            println!("error: {:?}", why);
        }

        println!("{}", elapsed);
    }
}
