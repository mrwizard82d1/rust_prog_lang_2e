// Taking a string slice reference provides a more general
// API that taking a String reference
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // The function, `first_word()`, works on slices of strings - whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("1. first word <{word}>");

    let word = first_word(&my_string[..]);
    println!("2. first word <{word}>");

    // `first_word()` also works on slices of string literals - again whether partial or whole
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    println!("3. first word <{word}>");

    let word = first_word(&my_string_literal[..]);
    println!("4. first word <{word}>");

    // Because string literals **are** string slices already, the following code works, too,
    // **without** the slice syntax!
    let word = first_word(my_string_literal);
    println!("5. first word <{word}>");
}
