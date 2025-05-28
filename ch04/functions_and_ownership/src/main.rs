fn main() {
    // The variable, `s`, of type, `String`, comes into scope
    let s = String::from("hello");
    
    // The value of `s` move into the function, `takes_ownership()`
    // The value of `s` will no longer be valid after `take_ownership()` returns
    takes_ownership(s);
    
    // The variable, `x`, of type `int32`, comes into scope
    let x = 5;
    
    // The variable, `x`, would **move** into the function, `makes_copy()`,
    // but the `Copy` trait is implemented for the `i32` type. Consequently,
    // the variable, `x`, is **still** owned by this function when 
    // `makes_copy()` returns.
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    // The variable, `some_string`, of type, `String`, comes into scope
    println!("{}", some_string);
    
    // When this function returns / prior to exiting the function, 
    // `some_string` goes out of scope and is **dropped**; that is, 
    // the `drop()` function is called to clean up.
}

fn makes_copy(some_integer: i32) {
    // The variable, `some_integer`, of type, `i32`, comes into scope.
    println!("{}", some_integer);
    
    // When this function returns / prior to exiting the function, 
    // `some_integer` goes out of scope; however, because values of 
    // type `i32`, are **copied** and not **moved**, no additional
    // clean up is needed.
}