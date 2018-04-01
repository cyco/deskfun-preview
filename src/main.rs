use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

extern crate byteorder;

mod my_byte_order;

mod game_data;
use game_data::ReadGameDataExt;

mod save_game;
use save_game::ReadSaveGameExt;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        show_usage();
    }

    let path = Path::new(&arguments[1]);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut data = file.read_game_data().unwrap();

    for i in 2..arguments.len() {
        let path = Path::new(&arguments[i]);
        println!("Reading save game from {}", path.display());
        let mut file = File::open(&path).unwrap();
        file.read_save_game(&mut data.zones).unwrap();

        let mut rest = Vec::new();
        let bytes_left = file.read_to_end(&mut rest).unwrap();
        assert!(
            bytes_left == 0,
            "{} bytes left after reading save game!",
            bytes_left
        );
    }

    exit(1);
}

fn show_usage() {
    println!("webfun-preview <game-data> [save-game ...]");
    exit(-1);
}
