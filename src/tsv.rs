use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents a `.tsv` file from https://github.com/MHohenberg/JPLT_Vocabulary
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JpltTsv {
    pub kanji: String,
    pub romanji: String,
    pub definition: String,
    pub details: Option<String>,
}

impl Default for JpltTsv {
    fn default() -> Self {
        Self {
            kanji: String::default(),
            romanji: String::default(),
            definition: String::default(),
            details: None,
        }
    }
}

impl JpltTsv {
    pub fn get_details(&self) -> String {
        todo!()
    }

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

pub fn load_from_tsv_file() -> Result<Vec<JpltTsv>> {
    // TODO use walkdir to load all .tsv files
    let path = "../jlpt-anki/jplt/tsv/jplt-n5_japanese_english.tsv";
    let mut rdr = csv::ReaderBuilder::new()
        // Source files are .tsv
        .delimiter(b'\t')
        // Source files do not have headers
        .has_headers(false)
        // Source files may or may not have a details column (my nomenclature)
        .flexible(true)
        .from_path(path)?;

    let mut records = Vec::new();

    // Serde and flexible parsing do not mix well so we will manually parse the fields
    for result in rdr.records() {
        let result = result?;

        let details = match result.get(3) {
            Some(details) => Some(details.to_string()),
            None => None,
        };

        let record = JpltTsv {
            kanji: result.get(0).unwrap().to_string(),
            romanji: result.get(1).unwrap().to_string(),
            definition: result.get(2).unwrap().to_string(),
            details,
        };

        records.push(record)
    }

    Ok(records)
}