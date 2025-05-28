fn main() {
    let mut s = String::from("hello");
    
    // First reference to `s`
    let r1 = &mut s;
    
    // Second reference to `s`
    let r2 = &mut s;
    
    println!("{r1}, {r2}");
}
