use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;

use anyhow::Result;
use tempfile::NamedTempFile;
use thiserror::Error;

use crate::mv;

#[derive(Debug, Error)]
pub enum ReplaceError {
    #[error("IO error: {msg}")]
    IOError { msg: String },
    #[error("FS error: {msg}")]
    FSError { msg: String },
}

pub mod var_replace {
    use regex::Regex;
    use std::ops::Range;
    use std::sync::LazyLock;

    static VARIABLE_EXPRESSION: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"x(?<num>\d+)").unwrap());

    pub trait ActingOn<T> {
        fn on(&self, obj: T) -> T;
    }

    pub struct VarReplacer {
        range: Range<u64>,
        name: String,
    }

    impl VarReplacer {
        pub fn new(range: Range<u64>, name: &str) -> VarReplacer {
            VarReplacer {
                range,
                name: name.to_string(),
            }
        }
    }

    impl ActingOn<String> for VarReplacer {
        fn on(&self, line: String) -> String {
            VARIABLE_EXPRESSION
                .replace_all(&line, |captures: &regex::Captures| {
                    let num = captures["num"].parse::<u64>().unwrap();
                    if !&self.range.contains(&num) {
                        captures.get(0).unwrap().as_str().to_string()
                    } else {
                        format!("{}[{}]", &self.name, num - &self.range.start)
                    }
                })
                .to_string()
        }
    }
}

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
        let file = File::open(&self.filename).map_err(|err| ReplaceError::IOError {
            msg: err.to_string(),
        })?;

        Ok(BufReader::new(file).lines())
    }

    pub fn print(&self) {
        for line in self.lines().unwrap().flatten() {
            println!("{line}");
        }
    }

    pub fn var_replace(&self, replacer: impl var_replace::ActingOn<String>) -> Result<()> {
        // Prepare temporary output file
        let mut temp_file = NamedTempFile::new().map_err(|err| ReplaceError::IOError {
            msg: format!("Failed to create temp file: {err}"),
        })?;

        // Perform operation line by line
        self.lines()?.flatten().try_for_each(|line| {
            temp_file
                .write_all(replacer.on(line).as_bytes())
                .and_then(|_| temp_file.write_all(b"\n"))
                .map_err(|err| ReplaceError::IOError {
                    msg: format!("Failed to write to temporary file: {err}"),
                })
        })?;

        // Substitute temporary output file with original file
        let temp_path = temp_file.into_temp_path();
        mv(temp_path.to_path_buf().as_path(), Path::new(&self.filename)).map_err(|err| {
            ReplaceError::FSError {
                msg: format!("Failed to update lp file: {}", err),
            }
        })?;

        Ok(())
    }
}
