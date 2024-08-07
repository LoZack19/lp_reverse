use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;

use tempfile::NamedTempFile;

use crate::mv;

#[derive(Debug)]
pub enum Error {
    Success,
    IOError { msg: String },
    FSError { msg: String },
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

    pub fn foo(&self) -> Result<()> {        
        let mut temp_file = NamedTempFile::new()
            .map_err(|err| Error::IOError { msg: format!("Failed to create temp file: {err}") })?;

        self.lines()?
            .flatten()
            .try_for_each(|line| {
                temp_file
                    .write_all(line.as_bytes())
                    .and_then(|_| temp_file
                        .write_all(b"\n")
                    )
                    .map_err(|err| Error::IOError { 
                        msg: format!("Failed to write to temporary file: {err}")
                    })
            })?;
        
        let temp_path = temp_file.into_temp_path();
        mv(temp_path.to_path_buf().as_path(), Path::new(&self.filename))
            .map_err(|err| Error::FSError { msg: format!("Failed to update lp file: {}", err) })?;

        Ok(())
    }
}
