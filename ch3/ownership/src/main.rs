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
}
