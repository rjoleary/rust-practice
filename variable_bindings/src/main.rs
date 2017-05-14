fn main() {
    // In these examples, types are inferred.
    let x = 5; // variable binding, we say that "x" is "binding"
    println!("x: {}", x);
    let (a, b) = (1, 2); // variable binding with pattern
    println!("a: {}, b: {}", a, b);

    // Types can be manually specified.
    let y: i32 = 5; // Integer types are [iu]{8,16,32,64}
    println!("y: {}", y);

    // Bindings are immutable. The following gives a compiler error:
    //y = 7;

    // Use the `mut` keyword to make bindings mutable.
    let mut z = 5;
    println!("z: {}", z);
    z = 7;
    println!("z: {}", z);

    // Scopes, blocks and shadows:
    let x = 7;
    println!("x: {}", x); // 7
    {
        println!("x: {}", x); // 7
        let x = 5;
        println!("x: {}", x); // 5
        let x = 3;
        println!("x: {}", x); // 3
    }
    println!("x: {}", x); // 7
}
