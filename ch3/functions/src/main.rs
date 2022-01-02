// `main` is an entry function where your program always starts its execution
fn main() {
    another_function(13);

    println!("Get five: {}", five());
}

// Functions are most important part of Rust
// You must always specify types of a function parameters and type of return value
fn another_function(x: i32) {
    println!("The value of x: {}", x);
}

// Rust is an expression based language
// You can return from a function early, using `return` keyword
// Or you can simply return last line of the function if you have it without a semicolon
fn five() -> i32 {
    5
}
