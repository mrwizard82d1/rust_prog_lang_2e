fn main() {
    // An example of `String::clone()`
    let s1 = String::from("hello");
    // Remember, a call to `clone` may be **expensive**
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
