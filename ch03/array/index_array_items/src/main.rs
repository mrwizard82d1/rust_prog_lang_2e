fn main() {
    let a = [0, 2, 4, 6, 8];

    let first = a[0];
    println!("First item in array {first}");

    let second = a[1];
    println!("Second item in array {second}");

    let last = a[a.len() - 1];
    println!("Last item in array {last}");
}
