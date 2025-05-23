fn main() {
    // addition ()
    let augend = 5;
    let addend = 10;
    let sum = augend + addend;
    println!("{} + {} = {}", augend, addend, sum);

    // subtraction
    let minuend = 95.5;
    let subtrahend = 4.3;
    let difference = 95.5 - 4.3;
    println!("{} - {} = {}", minuend, subtrahend, difference);

    // multiplication
    let multiplicand = 4;
    let multiplier = 30;
    let product = multiplicand * multiplier;
    println!("{} * {} = {}", multiplicand, multiplier, product);

    // division
    let dividend = 56.7;
    let divisor = 32.2;
    let quotient = dividend / divisor;
    println!("{} / {} = {}", dividend, divisor, quotient);

    // integer division (with negative dividend)
    let dividend = -5;
    let divisor = 3;
    let truncated = dividend / divisor;
    println!("{} / {} = {}", dividend, divisor, truncated);

    // remainder
    let dividend = 43;
    let divisor = 5;
    let remainder = dividend % divisor;
    println!("{} % {} = {}", dividend, divisor, remainder);
}
