fn main() {
    let s1 = String::from("Hello, world!");

    // use ampersand (`&`) to pass values by reference
    // Note: The opposite of referencing by using `&` is dereferencing,
    // which is accomplished with the dereference operator, `*`
    let s1_length = calc_length(&s1);
    println!("The length of s1: {}", s1_length);
}

fn calc_length(str: &String) -> usize {
    str.len()
}
