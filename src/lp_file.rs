use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
enum Error {
    Success,
    IOError { msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct LPFile {
    filename: String,
}

impl LPFile {
    pub fn new(filename: &str) -> LPFile {
        LPFile {
            filename: String::from(filename),
        }
    }

    fn lines(&self) -> Result<Lines<BufReader<File>>> {
        let file = File::open(&self.filename).map_err(|err| Error::IOError {
            msg: err.to_string(),
        })?;

        Ok(BufReader::new(file).lines())
    }

    pub fn print(&self) {
        for line in self.lines().unwrap().flatten() {
            println!("{line}");
        }
    }
}
