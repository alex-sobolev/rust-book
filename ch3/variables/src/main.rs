fn main() {
    // variables are immutable by default
    // use `mut` keyword to make them mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing: same variables names are allowed in one scope
    // they even might have different types, which you can't do with `mut`
    // PS with `mut` you can only reassign values to a variable of the same type
    let spaces = "     ";
    let spaces = spaces.len();

    println!("The length of spaces: {}", spaces); // "The length of spaces: 5"
}
