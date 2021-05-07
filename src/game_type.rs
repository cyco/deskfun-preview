use std::fmt;
use std::io;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameType {
    Yoda,
    Indy,
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match &self {
            GameType::Yoda => "Yoda Stories",
            GameType::Indy => "Indiana Jones and his Desktop Adventures",
        })
    }
}

pub trait ReadSaveGameTypeExt: io::Read {
    fn read_save_game_type(&mut self) -> io::Result<GameType> {
        let mut buffer = vec![0; 9];
        self.read_exact(&mut buffer)?;

        match String::from_utf8(buffer).expect("").as_str() {
            "YODASAV44" => Ok(GameType::Yoda),
            "INDYSAV44" => Ok(GameType::Indy),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Valid file header was not found.",
            )),
        }
    }
}

impl<R: io::Read + ?Sized> ReadSaveGameTypeExt for R {}
