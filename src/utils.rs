use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::Result;

fn get_jplt_dir() -> Result<PathBuf> {
    let dir = Path::new(env::current_dir()?.as_path()).join("jplt");

    Ok(dir)
}

fn get_dir(category: &str) -> Result<PathBuf> {
    let dir = get_jplt_dir()?.join(category);

    fs::create_dir(&dir)?;

    Ok(dir)
}

pub fn get_tsv_dir() -> Result<PathBuf> {
    Ok(get_dir("tsv")?)
}

pub fn get_anki_dir() -> Result<PathBuf> {
    Ok(get_dir("anki")?)
}