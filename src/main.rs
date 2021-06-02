extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    println!("Done!");
}

fn run() -> Result<(), Box<Error>> {
    let input_file_path = get_nth_arg(1)?;
    let output_file_path = get_nth_arg(2)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_path(input_file_path)
        .unwrap();
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output_file_path)
        .unwrap();
    let mut is_header_row = true;
    for result in rdr.records() {
        let mut record = result?;
        if is_header_row {
            record.push_field("Cloze Text");
            is_header_row = false;
        } else {
            record.push_field(cloze(record.get(0).unwrap().to_string()).as_str());
        }
        wtr.write_record(&record)?;
    }

    Ok(())
}

/// Returns the nth positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_nth_arg(n: usize) -> Result<OsString, Box<Error>> {
    match env::args_os().nth(n) {
        None => Err(From::from(format!("Problem with argument #{}.", n))),
        Some(file_path) => Ok(file_path),
    }
}

/// Converts a sentence into a cloze deletion suitable for Anki import.
fn cloze(sentence: String) -> String {
    let mut cloze = String::new();
    let mut new_word = true;
    let mut word_count = 1;

    for c in sentence.chars() {
        if new_word {
            cloze = format!("{}{{{{c{}::{}", cloze, word_count.to_string(), c);
            new_word = false;
            word_count += 1;
        } else if c == ' ' {
            cloze = format!("{}}}}}{}", cloze, c);
            new_word = true;
        } else if c == '.' {
            cloze = format!("{}}}}}{}", cloze, c);
        } else {
            cloze.push(c);
        }
    }
    cloze
}
