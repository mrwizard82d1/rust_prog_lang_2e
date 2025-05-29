fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn main() {
    let mut text = String::from("To be or not to be");

    let first_word = first_word(&text);

    println!("The first word of the text, <{text}> is <{first_word}>");

    println!("And now, clear the original text.");

    // Generates an error!
    text.clear();

    println!("The first word is **still available**: <{first_word}>");
}
