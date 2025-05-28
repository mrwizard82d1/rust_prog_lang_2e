fn main() {
    // The function `gives_ownership()` moves its return value into s1
    let s1 = gives_ownership();
    println!("{s1}");
    
    // The variable, `s2`, comes into scope
    let s2 = String::from("hello");
    println!("{s2}");
    
    // We now move `s2` into `takes_and_gives_back()`, which also
    // moves its return value into `s3`
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");
}
// Here, `s3` goes out of scope and is dropped. The variable `s2` was
// moved into `takes_and_gives_back()`, so nothing happens with it.
// Additionally, `s1` goes out of scope and is dropped.

fn gives_ownership() -> String {
    // The function, `gives_ownership`, will move its return value into
    // its caller.
    
    // The variable, `some_string`, comes into scope.
    let some_string = String::from("yours");
    
    some_string
    // The value of `some_string` is returned and moved out to the
    // calling function.
}

// This function takes a `String` and returns a `String`.
fn takes_and_gives_back(a_string: String) -> String {
    // The value of `a_string` from the caller is moved into 
    // this function, and so the caller will have ownership of it.
    
    a_string
    // The value of `a_string` is returned and is moved out to the
    // calling function.
}
