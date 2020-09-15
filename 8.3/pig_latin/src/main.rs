fn main() {
    let s = "The quick brown fox jumps over the lazy dog".to_string();
    println!("{}", s);
    
    let mut words: Vec<String> = Vec::new();
    let mut curr_word = String::new();
    
    /*
    Let's call doing this manually once a programming exercise.
    Next time I use split_whitespace.
    */
    for c in s.chars() {
        if c == ' ' {
            if curr_word.len() > 0 {
                words.push(curr_word);
                curr_word = String::new();
            }
        } else {
            curr_word.push(c);
        }
    }
    // check last word
    if curr_word.len() > 0 {
        words.push(curr_word);
    }
    
    for word in words.iter() {
        print!("{} ", pig_la(word));
    }
}

fn pig_la(word: &String) -> String {
    let mut pig = String::new();
    
    let mut chars = word.chars();
    let c = chars.next().unwrap();
    
    if is_vowel(c) {
        pig.push_str(word.as_str());
        pig.push_str("-hay");
    } else {
        pig.push_str(chars.as_str());
        pig.push('-');
        pig.push(c);
        pig.push_str("ay");
    }
    
    pig
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}