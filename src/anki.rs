use anyhow::Result;
use genanki_rs::{Deck, Field, Model, Note, Template};

use crate::tsv::{JlptLevel, Record};
use crate::utils;

fn create_anki_deck_model() -> Model {
    let custom_css = r#"
        .question {
            font-size: 5rem;
            font-weight: bold;
        }
    "#;

    Model::new_with_options(
        10000000,
        "JLPT Model",
        vec![Field::new("Question"), Field::new("Answer"), Field::new("Romanji")],
        vec![Template::new("JLPT Card")
            .qfmt(r#"<div class="question">{{Question}}</div>"#)
            .afmt(
                r#"
                    {{FrontSide}}
                    <hr id="answer">
                    {{Answer}}
                    <br><br>
                    {{Romanji}}"#
            )],
        Some(custom_css),
        None,
        None,
        None,
        None,
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
    let jlpt_level = &level.get_n_level();

    let deck_id = 11000000 + level.level as usize;

    let mut deck = Deck::new(deck_id,
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
            format!("{} ({})", definition, details)
        }
        None => {
            format!("{}\n", definition)
        }
    };

    let note = Note::new(model.to_owned(), vec![question, &answer, romanji])?;

    Ok(note)
}