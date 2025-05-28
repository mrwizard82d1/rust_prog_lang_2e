fn main() {
    let mut s = String::from("hello");
    
    {
        // A reference to a mutable `String` in a scope
        let r1 = &mut s;
        println!("r1 = {r1}");
    }

    // A reference to a mutable `String` since `r1` has gone out of scope
    let r2 = &mut s;
    println!("r2 = {r2}");
}
