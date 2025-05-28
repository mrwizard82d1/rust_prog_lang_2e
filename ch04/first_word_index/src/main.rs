fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // `enumerate()`` returns a tuple of index and character`
    // We desstcurcture the result of `enumerate()`
    // Because `enumerate()` returns a **reference** to the character
    // of the string at `index`, we use the reference operator, `&`
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn main() {
    let text = String::from("To be or not to be");
    // let text = String::from("supercalifragilisticexpialidocious");
    let after_first_word = first_word(&text);

    println!("Testing: <{text}>");
    println!("First word between indices 0 and {after_first_word}");
}
