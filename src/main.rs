fn main() {
    let example_sentence_en = "I am good.".to_string();
    let example_sentence_tp = "mi pona.".to_string();


    let result = cloze(example_sentence_en);

    println!("{}", result);
}

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

