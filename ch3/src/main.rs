fn main() {
    variables();
}

fn variables() {
    // variables are immutable by default
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // error: cannot assign twice to immutable variable `x`

    // use `mut` keyword to make a variable mutable
    let mut y = 5;
    println!("The value of y is: {y}");

    y = 6;
    println!("The value of y is: {y}");

    // constants are always immutable
    // their type must be always annotated
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    // shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {z}");

    // shadowing allows to change the type of the variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // shadowing is different from `mut`
    // shadowing allows to change the type of the variable
    // shadowing allows to change the mutability of the variable
    let mut spaces = "   ";
    // spaces = spaces.len(); // error: mismatched types
    println!("The value of spaces is: {spaces}");
}
