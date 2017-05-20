fn main() {
    // Booleans: true or false
    let x: bool = true;
    let y = false;
    println!("Booleans: {}, {}", x, y);

    // Characters: single unicode character, four bytes
    let x: char = 'x';
    let smiley = '☺';
    println!("Characters: {}, {}", x, smiley);

    // Numeric types: {i,u,f}{8,16,32,64,size}
    // - floats are either f32 or f64
    // - isize and usize depend on the underlying machine architecture
    let x = 42; // defaults to i32
    let y = 1.0; // defaults to f64
    println!("Numeric: {} {}", x, y);

    // Arrays: size fixed at compile time
    let a = [1, 2, 3];
    let b = ['a'; 20]; // shortcut for repetition
    println!("len: {}", a.len());
    println!("{} {} {}", a[0], a[1], a[2]);
    println!("len: {}", b.len());

    // Slices: pointer to beginning of data and length
    let c = &a[..]; // contains all elements of a
    let d = &a[1..]; // contains all but the first element
    println!("Slice lengths: {} {}", c.len(), d.len());

    // str: most primitive string type, covered later

    // Tuples: ordered list of fixed size
    let e = (1, "hello");
    let f: (i32, &str); // unbound
    f = e; // must be same type of tuple
    let (g, h) = f; // destructuring let
    println!("Tuple: ({}, {})", g, h);
    let i = (7,); // single tuple
    println!("Single tuple: ({},)", i.0);
    let j = (11, 22, 33);
    println!("Tuple indexing: ({}, {}, {})", j.0, j.1, j.2);

    // Functions
    fn foo(x: i32) -> i32 { x }
    let k: fn(i32) -> i32 = foo;
    println!("{}", k(100))
}
