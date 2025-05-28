fn main() {
    // A mutable string
    let mut s = String::from("hello");

    // `push_shr()` appends a literal to a `String``
    s.push_str(", Rust String world!");

    // Prints "hello, Rust String world!"
    println!("{s}");
}
