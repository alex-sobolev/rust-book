fn main() {
    // A string slice is a reference to part of a String
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello);
    println!("{}", world);
}
