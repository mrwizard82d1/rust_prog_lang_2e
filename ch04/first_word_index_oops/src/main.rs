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
    let mut s = String::from("hello world");

    // `after_first_word` will be 5
    let after_first_word = first_word(&s);
    println!("First word between indices 0 and {after_first_word}");

    // However, we can "empty" the `String` making it equal to ""
    s.clear();
    println!("The index after the first word is still {after_first_word}");
    println!("However, the string is now **empty**: <{s}>");
    println!("So the index is **meaningless**.")

}
