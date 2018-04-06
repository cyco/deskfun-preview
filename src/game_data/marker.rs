use std::io;
use std::io::Read;

pub trait ReadMarkerExt: io::Read {
    fn read_category_marker(&mut self, name: &str) -> io::Result<()> {
        let mut marker = String::with_capacity(4);
        self.take(4).read_to_string(&mut marker)?;
        if marker != name {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Expected to find {} category, found {} instead",
                    name, marker
                ),
            ))
        } else {
            Ok(())
        }
    }
}

impl<R: io::Read + ?Sized> ReadMarkerExt for R {}
