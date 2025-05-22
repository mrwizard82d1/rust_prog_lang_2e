fn main() {
    let x = 5;
    println!("Declare a variable named x = {x}.");

    println!("Shadow this variable with **another** variable named x whose value is x + 1 = {}.", x + 1);
    let x  = x + 1;
    {
        let x = x * 2;
        println!("Shadow x again. It's value in the **inner** scope is {x}.");
    }
    println!("The value of x in the **outer** scope is visible {x}!");
}
