fn main() {
    print_number(7);
    print_sum(3, 5);
    print_number(add_one(9));

    // Function pointers
    let f = add_one;
    let mut g: fn(x: i32) -> i32;
    g = add_one;
    print_number(f(g(10)));

    let mut x = 10;
    let y = x = 5;
    // x now has the value 5.
    // y now has the value (), the empty tuple.
    // This quirk is required for the borrow checker.

    diverge();

    // A diverging function can be used as any type:
    let x: i32 = diverge();
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    // Expression statement not followed by a newline is treated as a return.
    x + 1
}

fn foo(x: i32) -> i32 {
    // "return" keyword must be used for an early return.
    return x;

    x + 1
}

// The exclamation point indicates this function will never return.
fn diverge() -> ! {
    // Set RUST_BACKTRACE=1 environment variable for more information.
    panic!("This function never returns!")
}
