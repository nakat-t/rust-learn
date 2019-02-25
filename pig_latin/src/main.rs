fn pig_latin(word: &str) -> String {
    let vowel = ['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', 'Y'];
    let mut chars = word.chars();
    let first = chars.next();
    return match first {
        Some(ch) if vowel.contains(&ch) => format!("{}-hay", word),
        Some(ch) if ch.is_alphabetic() => format!("{}-{}ay", chars.collect::<String>(), ch),
        _ => String::from(word)
    }
}

fn pig_latin_print(str: &str) {
    let pigs: Vec<String> = str.split_whitespace().map(|w| pig_latin(w)).collect();
    println!("{}", pigs.join(" "));
}

fn main() {
    pig_latin_print("first apple");
    pig_latin_print("Pig Latin is mainly used in fun");
}
