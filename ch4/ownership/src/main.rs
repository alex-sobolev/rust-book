fn main() {
    // `stack` vs `heap`

    // Stack:
    // you store data which size is known and fixed at compile time:
    // `&str`, int, float, array etc

    // Heap:
    // you store data which size is not known or may change after compilation:
    // String, vector etc

    // Stack: first in, last out
    // pushing onto the stack (adding to the end), popping off the stack (removing the head)

    // Heap: allocating on the heap
    // the allocator must first find a big enough space to hold the data and
    // then perform bookkeeping to prepare for the next allocation
    // Once space was found and reserved for a given data, the allocator returns a pointer
    // A pointer is an address, at which the dynamically allocated data is stored
    // A pointer has a fixed size, so it can be stored on the stack
    // To access the data, you must follow the pointer
    // This is why accessing data in the heap is slower than accessing data on the stack

    let mut s = String::from("Hello");

    s.push_str(", world!");
    println!("{}", s);

    // In Rust, each data has one owner, i.e. it belongs to only one variable

    // Stack-only data implements Copy trait
    // It means if you try to assign one variable value to another, it will be cloned:
    let val1 = 12;
    let val2 = val1; // 12 is cloned here, because it's on the stack, it's fast to do.

    // Heap-based data doesn't implement `Copy`, so if you want to clone it, you have to do it explicitly:
    let val1 = String::from("Hello, Jack!");
    let val2 = val1.clone();

    // In any other case, a `move` of heap data will happen:
    let val1 = String::from("Hello, Jack!");
    let val2 = val1; // a reference to a given heap data (pointer, len, capacity) was moved from `val1` to `val2`
                     // `val1` is no longer valid from here on, and cannot be accessed after the `move` operation

    // The same logic of `move` / `copy` applies when you pass arguments to a function
    let string1 = String::from("My awesome string");
    take_ownership(string1); // `string1` is moved into the function `take_ownership` and can no longer be used

    let num1 = 1;
    make_copy(num1); // Since `i32` type implements `Copy`, it's cloned to the `make_copy` function
                     // And `num1` can still be used after calling the `make_copy` function

    // returning a value from a function can also pass ownership of the data it returns:
    let another_fn_string = give_ownership();
    println!("{}", another_fn_string); // "A string from another function"

    // Also, a function can take ownership and return it back:
    let my_str = String::from("A new string here");
    let str_returned_back = take_and_give_ownership(my_str); // `my_str` was move into the function and no longer can be used
    println!("{}", str_returned_back); // "A new string here"
}

fn take_ownership(str: String) {
    // take_ownership takes ownership of the argument of type `String` passed to it
    println!("{}", str);
    // And then `str` is dropped here (at the end of the current scope)
}

fn make_copy(n: i32) {
    // Since `i32` type implements `Copy`, it's cloned to the `make_copy` function
    println!("{}", n);
}

fn give_ownership() -> String {
    let some_string = String::from("A string from another function");

    some_string // returns data of `String` type here
}

fn take_and_give_ownership(str: String) -> String {
    str
}
