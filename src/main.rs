use std::{env, process};

use anyhow::Result;

use jplt_anki::tsv;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    let levels = tsv::load_tsv_files()?;

    Ok(())
}
