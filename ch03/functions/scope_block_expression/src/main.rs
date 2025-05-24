fn main() {
    // A scope block is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}
