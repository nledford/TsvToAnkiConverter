use std::{env, process};

use anyhow::Result;

use jplt_anki::tsv;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    let levels = match tsv::load_tsv_files() {
        Ok(levels) => levels,
        Err(err) => {
            println!("Error occurred while loading data from .tsv files: {}", err);
            process::exit(1)
        }
    };

    println!("\n\nDone!");

    Ok(())
}
