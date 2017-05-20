fn main() {
    // You can either use square [] or round () brackets for a macro. By convention, vec! uses
    // square and prinln uses square.
    let v = vec![1, 2, 3, 4, 5];

    // Repeat initial value.
    let w = vec![4; 10];

    // Vec<T> employs generic. The size of T must be known at compile time. If the size is unknown,
    // T can be boxed.

    // Use the subscript operator to access elements.
    println!("Length of v: {}", v.len());
    println!("Length of w: {}", w.len());
    println!("v[0]: {}", v[0]); // indexing uses the usize type
    println!("v[1]: {}", v[1]);

    // Safely check for out of bounds access.
    let v = vec![1, 2, 3];
    match v.get(20) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    // Three ways to iterate.
    let mut x = vec![1, 2, 3];
    for i in &x { // iterate over references
        println!("{}", i);
    }
    for i in &mut x { // iterate over mutable references
        println!("{}", i);
    }
    for i in x { // take ownership and iterate, x cannot be used again
        println!("{}", i);
    }

    // Out-of-bounds access
    println!("v[20]: {}", v[20]); // run-time panic
}
