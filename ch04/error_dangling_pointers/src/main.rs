fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // `dangle()` return a reference to a String

    // `s` is a new `String`
    let s = String::from("hello");

    // We return a reference to the string, `s`
    &s
}
// Here, `s` goes out of scope and is dropped so its memory goes away.
// Danger, danger, dange, Will Robinson!
