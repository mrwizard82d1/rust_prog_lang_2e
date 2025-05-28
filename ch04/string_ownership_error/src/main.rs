fn main() {
    let s1 = String::from("hello");
    // The variable, s1, **no longer owns** the string
    // This action is called a **move**
    let s2 = s1;  // s1 **no longer owns** the string


    println!("{s1}, Rust String Error world1");
}
