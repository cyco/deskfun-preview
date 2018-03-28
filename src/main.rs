use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::exit;

extern crate byteorder;

mod game_data;
use game_data::ReadGameDataExt;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        show_usage();
    }

    let path = Path::new(&arguments[1]);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    file.read_game_data();

    exit(1);
}

fn show_usage() {
    println!("webfun-preview <game-data> [save-game]");
    exit(-1);
}
