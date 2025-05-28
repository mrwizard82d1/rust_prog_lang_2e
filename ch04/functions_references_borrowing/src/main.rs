fn main() {
    let s1 = String::from("hello");
    
    // The `&s1` syntax is a reference to `s1` but // **does not** own it. 
    // Because the value is not owned by the called function, the value
    // that it references **will not** be dropped when the refence is
    // no longer used.
    let len = calculate_length(&s1);
    
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    // `s` is a **reference** to a `String`.
    s.len()
}
// Here, `s` goes out of scope. But because it does not have ownership of
// the value to which it refers, the `String` value is not dropped when
// `s` goes out of scope.