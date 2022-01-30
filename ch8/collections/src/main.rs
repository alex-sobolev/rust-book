fn main() {
    // Collections are stored on the heap
    // They can grow or shrink in size

    // 1. Vectors
    // 1.1. Create a vector, using an associated method `new` (aka static method):
    let mut vec1: Vec<u32> = Vec::new();

    vec1.push(1);
    println!("vec1: {:?}", vec1);

    // 1.2. Create a vector, using `vec!` macro:
    let mut vec2: Vec<String> = vec![String::from("Hello, Jack!")];

    println!("vec2: {:?}", vec2);

    // 1.3. Accessing elements in a vector:
    let vec3 = vec![1, 2, 3, 4, 5];

    // access by indexing
    // if an elem doesn't exist, a program will panic
    let elem3: &i32 = &vec3[2];

    println!("elem3: {}", elem3);

    // access by `get` method which returns `Option<T>`:
    let elem2 = match vec3.get(1) {
        Some(num) => println!("elem2: {}", num),
        None => println!("elem2 doesn't exist"),
    };

    // iterating over a vector:
    for (i, num) in vec3.iter().enumerate() {
        println!("elem {}: {}", i, num);
    }

    // iterate over mutable references:
    let mut vec4 = vec![100, 200, 300, 400, 500];

    for num in &mut vec4 {
        // since a reference to an element is returned rather than an element itseld
        // you need to dereference a reference first with deref operator `*`:
        *num += 50;
        println!("{}", num);
    }

    // Store elements of different types in a vector, using Enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.13),
        SpreadsheetCell::Text(String::from("Hello, world!")),
    ];

    for cell in &row {
        println!("cell: {:?}", cell);
    }
}
