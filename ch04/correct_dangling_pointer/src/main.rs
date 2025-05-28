fn main() {
    let correct_reference = no_dangle();

    println!("{correct_reference}");
}

fn no_dangle() -> String {
   let s = String::from("hello");

    s
}
