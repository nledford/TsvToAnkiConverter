use std::process;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::utils;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JlptLevel {
    pub level: i32,
    pub records: Vec<Record>,
}

impl JlptLevel {
    pub fn get_n_level(&self) -> String {
        format!("N{}", self.level)
    }
}

/// Represents a `.tsv` file from https://github.com/MHohenberg/JPLT_Vocabulary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub kanji: String,
    pub romanji: String,
    pub definition: String,
    pub details: Option<String>,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            kanji: String::default(),
            romanji: String::default(),
            definition: String::default(),
            details: None,
        }
    }
}

impl Record {
    pub fn print(&self) {
        println!("Kanji: {}", self.kanji);
        println!("Romanji: {}", self.romanji);
        print!("Definition: {} ", self.definition);

        match &self.details {
            Some(details) => {
                print!("({})", details)
            }
            None => ()
        }
    }
}

fn get_level(file_name: &str) -> i32 {
    if file_name.contains('1') {
        1
    } else if file_name.contains('2') {
        2
    } else if file_name.contains('3') {
        3
    } else if file_name.contains('4') {
        4
    } else {
        5
    }
}

pub fn load_tsv_files() -> Result<Vec<JlptLevel>> {
    let mut levels = Vec::new();

    let path = utils::get_tsv_dir()?;
    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.path().is_dir() {
            continue;
        }

        let file_name = entry.file_name().to_str().unwrap();
        let jlpt_level = get_level(file_name);

        println!("Level: {}", &jlpt_level);

        println!("Fetching data from {}...", entry.path().display());
        let records = load_from_tsv_file(entry.path().to_str().unwrap(), jlpt_level)?;

        let level = JlptLevel { level: jlpt_level, records };
        levels.push(level)
    }

    Ok(levels)
}

fn load_from_tsv_file(file: &str, level: i32) -> Result<Vec<Record>> {
    let mut rdr = csv::ReaderBuilder::new()
        // Source files are .tsv
        .delimiter(b'\t')
        // Source files do not have headers
        .has_headers(false)
        // Source files may or may not have a details column (my nomenclature)
        .flexible(true)
        .from_path(file)?;

    let mut records = Vec::new();

    // Serde and flexible parsing do not mix well so we will manually parse the fields
    for result in rdr.records() {
        let result = result?;

        let kanji = result.get(0).unwrap();

        let mut details = result.get(3).map(|details| details.to_string());

        let mut definition = match result.get(2) {
            Some(definition) => definition.to_string(),
            None => {
                println!("\nCannot find definition for word: {}", kanji);
                println!("Current Level: N{}\n", level);
                process::exit(1)
            }
        };

        if definition.is_empty() && details.is_some() {
            definition = details.unwrap();
            details = None;
        }

        let record = Record {
            kanji: kanji.to_string(),
            romanji: result.get(1).unwrap().to_string(),
            definition,
            details,
        };

        records.push(record)
    }

    Ok(records)
}