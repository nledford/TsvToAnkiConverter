use std::{env, process};

use anyhow::Result;

use jplt_anki::{anki, tsv};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    let levels = match tsv::load_tsv_files() {
        Ok(levels) => levels,
        Err(err) => {
            println!("Error occurred while loading data from .tsv files: {}", err);
            process::exit(1)
        }
    };

    match anki::create_anki_decks(levels) {
        Ok(_) => {
            println!("Decks created successfully!")
        },
        Err(err) => {
            println!("Error occurred while creating anki decks: {}", err);
            process::exit(1)
        }
    }

    println!("\n\nDone!");

    Ok(())
}
