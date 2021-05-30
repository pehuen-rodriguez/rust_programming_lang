use std::io;

fn main() {
    loop {
        println!("What would you say?");
        let mut sentence = String::new();
        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");
        let sentence = sentence.trim();
        let mut modified_sentence: Vec<String> = Vec::new();

        for word in sentence.split_whitespace() {
            let mut word_iter = word.chars();
            let first_char = word_iter.next();
            let mut _modified_word = String::new();

            if is_vowal(first_char) {
                _modified_word = format!("{}-hay", word);
            } else {
                let s: String = word_iter.into_iter().collect();
                _modified_word = format!("{}-{}ay", s, first_char.unwrap());
            }
            modified_sentence.push(_modified_word);
        }
        println!("No, you meant: {}", modified_sentence.join(" "));
    }
}

fn is_vowal(first_char: Option<char>) -> bool {
    match first_char {
        Some('a') => true,
        Some('e') => true,
        Some('i') => true,
        Some('o') => true,
        Some('u') => true,
        _ => false,
    }
}
