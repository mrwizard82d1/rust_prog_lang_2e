fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);
    
    println!("The length of '{s2}' is {len} characters.");
}

fn calculate_length(s: String) -> (String, usize) {
    // `String::len()` returns the length of a String
    let length = s.len();

    (s, length)
}