use std::env;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use std::io::prelude::*;
use std::io;

extern crate byteorder;

mod game_data;

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

    let mut game_data = match game_data::GameData::new(&mut file) {
		Err(why) => panic!("couldn't read file {} {}", path.display(), why.description()),
        Ok(game_data) => game_data,
	};
    println!("{}", game_data);

    exit(1);
}

fn show_usage() {
    println!("webfun-preview <game-data> [save-game]");
    exit(-1);
}
