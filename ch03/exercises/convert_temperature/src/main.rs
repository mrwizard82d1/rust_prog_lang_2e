use std::io;

fn main() {
    println!("Convert between temperature on different scales");

    println!("Enter the from temperature");
    let mut from_temperature_magnitude = String::new();
    io::stdin()
        .read_line(&mut from_temperature_magnitude)
        .expect("Failed to read from temperature magnitude");
    println!("Converting from temperature {from_temperature_magnitude}");
    let from_temperature_magnitude : f64= from_temperature_magnitude
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("Enter the from temperature scale (C or F)");
    let mut from_temperature_scale = String::new();
    io::stdin()
        .read_line(&mut from_temperature_scale)
        .expect("Failed to read from temperature scale");
    println!("Converting from temperature scale {from_temperature_scale}");
    let from_temperature_scale = from_temperature_scale
        .trim()
        .to_uppercase();
    if from_temperature_scale == "C" {
        let to_temperature_magnitude = (9.0 / 5.0)  * from_temperature_magnitude + 32.0;
        println!("{from_temperature_magnitude} C is {to_temperature_magnitude} F");
    } else if from_temperature_scale == "F" {
        let to_temperature_magnitude = (from_temperature_magnitude - 32.0) * (5.0 / 9.0);
        println!("{from_temperature_magnitude} F is {to_temperature_magnitude} C");
    } else {
        println!("Unrecognized temperature scale {from_temperature_scale}. Bad human!")
    }
}
