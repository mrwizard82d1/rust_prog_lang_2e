fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // No problem
    let r2 = &s; // Also no problem
    println!("{r1} and {r2}");

    // Since variables `r1` and `r2` are not used after this point,
    // we can introduce a mutual reference to the value `s`.
    let r3 = &mut s;
    println!("{r3}");
}
