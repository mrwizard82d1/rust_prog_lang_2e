fn main() {
    let mut s = String::from("hello");

    let r1 = &s;  // Not a problem
    let r2 = &s;  // Also not a problem

    // BIG PROBLEM
    let r3 = &mut s;

    println!("{r1}, {r2}, and {r3}");
}
