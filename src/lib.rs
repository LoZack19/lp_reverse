use anyhow::Result;
use rand::Rng;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub mod lp_file;

pub fn append_to_filename(path: &Path, suffix: impl AsRef<OsStr>) -> PathBuf {
    let mut path = path.as_os_str().to_os_string();
    path.push(suffix);
    path.into()
}

pub fn generate_unique_name(base_path: &Path) -> PathBuf {
    let mut rng = rand::thread_rng();
    let filename_without_extension = base_path.with_extension("");

    loop {
        let random_suffix = rng.gen_range(1000..10000).to_string();
        let new_filename = append_to_filename(&filename_without_extension, random_suffix);

        if !new_filename.exists() {
            return new_filename;
        }
    }
}

fn mv(from: &Path, into: &Path) -> Result<()> {
    let unique_name = generate_unique_name(&into);
    fs::rename(&from, &unique_name)
        .and_then(|_| fs::remove_file(&into))
        .and_then(|_| fs::rename(unique_name, &into))?;
    Ok(())
}
