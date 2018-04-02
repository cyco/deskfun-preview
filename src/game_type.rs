use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub enum GameType {
    Yoda,
    Indy,
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match &self {
            Yoda => "Yoda Stories",
            Indy => "Indiana Jones and his Desktop Adventures",
        })
    }
}

pub fn identify_save_game_type(raw_path: String) -> io::Result<GameType> {
    let path = Path::new(&raw_path);
    let mut file_magic = String::new();
    File::open(&path)?.take(9).read_to_string(&mut file_magic)?;

    match file_magic.as_str() {
        "YODASAV44" => Ok(GameType::Yoda),
        "INDYSAV44" => Ok(GameType::Indy),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Valid file header was not found.",
        )),
    }
}
