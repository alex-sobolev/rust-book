fn main() {
    let s1 = String::from("Hello, world!");

    // use ampersand (`&`) to pass values by reference
    // Note: The opposite of referencing by using `&` is dereferencing,
    // which is accomplished with the dereference operator, `*`
    let s1_length = calc_length(&s1);
    println!("The length of s1: {}", s1_length);

    // references are immutable by default
    // if you want to modify the value of the referenced data,
    // you need to explicitly use `mut`, as with other variables
    let mut s2 = String::from("Original part");

    change_str(&mut s2);
    println!("{}", s2); // "Original part, modified part"

    // Important rules for references:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references;
    // 2. References must always be valid (the compiler won't allow dangling pointers)
}

fn calc_length(str: &String) -> usize {
    str.len()
}

fn change_str(str: &mut String) {
    str.push_str(", modified part");
}
