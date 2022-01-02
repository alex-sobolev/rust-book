fn main() {
    // 3 types of loops: `loop`, `while`, `for`

    // `loop`
    simple_loop();
    nested_loops();
    return_from_loop();

    // `while`
    while_loop();

    // `for`
    for_loop();
    for_loop_with_count();

    let n = 13;
    let fib = fibonacci(n);
    println!("The fibonacci of {} is: {}", n, fib);
}

fn simple_loop() {
    let mut x = 0;

    // loop is an infinite loop
    loop {
        if x > 9 {
            println!("reached the limit");
            break;
        }

        println!("again!");
        x += 1;
    }
}

// you can nest loops inside one another
// and you can use labels for loops to `break` or `continue` them
// by default if you just use `break` / `continue`: it's applied to the innermost loop
fn nested_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("Outer loop count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);

            if remaining == 5 {
                break;
            }

            if count == 3 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
}

// loops are also expressions in Rust
// you can assign them to variables
fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 9 {
            break counter * 10;
        }
    };

    println!("The result: {}", result);
}

// `while` is a conditional loop
// you achieve the same result with `loop` and `if`, `else`, `break` keywords
// but `while` is more concise
fn while_loop() {
    let mut num = 9;

    while num >= 0 {
        println!("The number: {}", num);

        num -= 1;
    }

    println!("Lift off!");
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for elem in arr {
        println!("The array current number: {}", elem);
    }
}

fn for_loop_with_count() {
    // you can use `Range` type
    for count in (1..4).rev() {
        println!("{}", count);
    }

    println!("Lift off!");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut x1: u32 = 1;
    let mut x2: u32 = 1;
    let mut x3: u32 = x1 + x2;

    for i in 3..=n {
        x3 = x2 + x1;
        x1 = x2;
        x2 = x3;
    }

    return x3;
}
