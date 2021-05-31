extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let example_sentence_en = "I am good.".to_string();
    let example_sentence_tp = "mi pona.".to_string();

    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        //.from_reader(io::stdin());
        .from_path(file_path)
        .unwrap();
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    let result = cloze(example_sentence_en);

    println!("{}", result);

    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

//fn import_file(filepath: &str) ->

fn cloze(sentence: String) -> String {
    let mut cloze = String::new();
    let mut new_word = true;
    let mut word_count = 0;

    for c in sentence.chars() {
        if new_word {
            cloze = format!("{}{{c{}::{}", cloze, word_count.to_string(), c);
            new_word = false;
            word_count += 1;
        } else if c == ' ' {
            cloze = format!("{}}}{}", cloze, c);
            new_word = true;
        } else if c == '.' {
            cloze = format!("{}}}{}", cloze, c);
        } else {
            cloze.push(c);
        }
    }
    cloze
}
