use jplt_anki::tsv;
use std::process;

fn main() {
    match tsv::load_from_tsv_file() {
        Ok(records) => {
            println!("Total records: {}", records.len());

            let details_record = records.get(326).unwrap();
            details_record.print()
        }
        Err(err) => {
            println!("Error loading tsv file: {}", err);
            process::exit(1)
        }
    }
}
