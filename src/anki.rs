use anyhow::Result;
use genanki_rs::{Deck, Field, Model, Note, Template};
use rand::Rng;

use crate::tsv::{JlptLevel, Record};
use crate::utils;

fn get_anki_id() -> usize {
    let mut rng = rand::thread_rng();
    let id:i32 = rng.gen_range(0..99999999);

    id as usize
}

fn create_anki_deck_model() -> Model {
    Model::new(
        get_anki_id(),
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    )
}

pub fn create_anki_decks(levels: Vec<JlptLevel>) -> Result<()> {
    println!("Creating anki decks...");

    let model = create_anki_deck_model();

    for level in levels {
        println!("Current level: {}", &level.level);
        create_anki_deck(&model, &level)?
    }

    Ok(())
}

fn create_anki_deck(model: &Model, level: &JlptLevel) -> Result<()> {
    let jlpt_level = &level.level;

    let mut deck = Deck::new(get_anki_id(),
                             &format!("Japanese {}", jlpt_level),
                             &format!("Deck for studying Japanese vocabulary at the {} level", jlpt_level),
    );

    for record in &level.records {
        let note = create_anki_note(model, record)?;
        deck.add_note(note)
    }

    // Save deck to file
    let file_name = format!("Japanese {}.apkg", jlpt_level);
    let anki_path = utils::get_anki_dir()?;
    let final_path = anki_path.join(file_name);

    deck.write_to_file(final_path.to_str().unwrap())?;

    Ok(())
}

fn create_anki_note(model: &Model, record: &Record) -> Result<Note> {
    let Record {
        kanji,
        romanji,
        definition,
        details,
        ..
    } = record;

    let question = kanji;
    let answer = match details {
        Some(details) => {
            format!("{} ({})\n{}", definition, details, romanji)
        }
        None => {
            format!("{}\n{}", definition, romanji)
        }
    };

    let note = Note::new(model.clone(), vec![question, &answer])?;

    Ok(note)
}