fn main() {
    // `stack` vs `heap`

    // Stack:
    // you store data which size is known and fixed at compile time (scalar types):
    // `&str`, int, float, array etc

    // Heap:
    // you store data which size is not known or may change after compilation (compound types):
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
}
