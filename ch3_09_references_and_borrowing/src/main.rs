fn main() {
    let a = vec![2, 3, 4];
    let b = vec![1, 2, 3];
    println!("{}", dot3(&a, &b));
    println!("{}", dot3(&a, &b));

    // Using mutable references:
    let mut x = 7;
    { // A new scope is necessary!
        let y = &mut x;
        *y = *y + 1;
    }
    println!("{}", x);

    // Another example:
    let mut v = vec![1, 2, 3];
    incr(&mut v);
    println!("{} {} {}", v[0], v[1], v[2]);

    // The Rules:
    //
    // 1. Any borrow must last for a scope no greater than that of the owner.
    // 2. You may have one or the other of these two kinds of borrows, but no both at the same
    //    time:
    //   a) one or more references (&T) to a resource,
    //   b) exactly one mutable reference (&mut T).
}

// The & type means its a reference.
fn dot3(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    use std::cmp;
    let mut result = 0;
    for i in 0..cmp::min(a.len(), b.len()) {
        result += a[i] * b[i];
    }
    result
}

// Mutable references let you modify the resource.
fn incr(v: &mut Vec<i32>) {
    for x in v {
        *x += 1;
    }
}
