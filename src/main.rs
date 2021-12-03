extern crate copypasta;
extern crate csv;

use crate::copypasta::ClipboardProvider;
use copypasta::ClipboardContext;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    println!("Done!");
}

fn run() -> Result<(), Box<Error>> {
    let args = std::env::args();
    if args.len() < 2 {
        return Err(
            "What? How did this happen? You had too few arguments to run the program!".into(),
        );
    } else if args.len() == 2 {
        // Read from clipboard and output to clipboard

        // First, get the clipboard's contents
        let mut ctx = ClipboardContext::new().unwrap();
        // println!("{:?}", ctx.get_contents().unwrap());

        // Convert the contents into a vec
        let contents = ctx.get_contents().unwrap();
        let contents = contents.trim();
        let content_vec: Vec<&str> = contents.split("\r\n").collect();

        // Iterate over the vec, converting each element into a group of hinted cloze deletions
        let mut cloze_deletions: Vec<String> = Vec::new();
        for line in content_vec.iter() {
            let mut counter = 0;
            let mut cloze_deletion = String::new();
            let cloze_deletion_iter = line.split(" ");

            // Make hinted cloze deletion
            for word in cloze_deletion_iter {
                // println!("{}", word);
                counter += 1;

                // Split first character from the rest of the word
                let first_char = word.chars().next().unwrap();
                let rest_of_word = word.chars().skip(1).collect::<String>();

                cloze_deletion.push_str(&format!("{}", first_char));
                cloze_deletion.push_str("{{c");
                cloze_deletion.push_str(counter.to_string().as_str());
                cloze_deletion.push_str(":");
                cloze_deletion.push_str(&rest_of_word);
                cloze_deletion.push_str("}}");
                cloze_deletion.push_str(" ");
            }
            // println!("{}", cloze_deletion);
            cloze_deletions.push(cloze_deletion.trim().to_string());
        }

        // Iterate over the vec, converting each element into a group of standard cloze deletions
        // TODO: This is a repeat of the above loop, can it be fixed?
        for line in content_vec.iter() {
            let mut counter = 0;
            let mut cloze_deletion = String::new();
            let cloze_deletion_iter = line.split(" "); // TODO: Should split_whitespace() be used here?
            let mut cloze_deletion = String::new();
            // Make standard cloze deletion
            for word in cloze_deletion_iter {
                // println!("{}", word);
                counter += 1;
                cloze_deletion.push_str("{{c");
                cloze_deletion.push_str(counter.to_string().as_str());
                cloze_deletion.push_str(":");
                cloze_deletion.push_str(word);
                cloze_deletion.push_str("}}");
                cloze_deletion.push_str(" ");
            }
            // println!("{}", cloze_deletion);
            cloze_deletions.push(cloze_deletion.trim().to_string());
        }

        // Turn the vec of cloze deletions back into a string
        let mut result_string = String::new();
        for cloze in cloze_deletions.iter() {
            result_string.push_str(cloze.as_str());
            result_string.push_str("\r\n");
        }

        // Then, write the contents to the clipboard
        ctx.set_contents(result_string).unwrap();
    } else if args.len() == 3 {
        // Read from file and output to clipboard
        unimplemented!();
    } else if args.len() == 4 {
        // Read from file and output file
        let input_file_path = get_nth_arg(2)?;
        let output_file_path = get_nth_arg(3)?;

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

        return Ok(());
    } else {
        return Err("Too many arguments".into());
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
