fn print3(v: Vec<i32>) -> Vec<i32> {
    println!("{} {} {}", v[0], v[1], v[2]);
    v
}

fn main() {
    // Rust ensures there's always one binding to any given resource.
    let v = vec![1, 2, 3];
    println!("{} {} {}", v[0], v[1], v[2]);
    // Move v into w.
    let w = v;
    // The following uses v after it is moved and is a compile error:
    //println!("{} {} {}", v[0], v[1], v[2]);

    print3(w);
    // Likewise, the following is a compile error:
    //print3(w);

    // Handing ownership back from a function.
    let v = vec![1, 2, 3];
    let v = print3(v);
    print3(v);
    // This gets tedious, so see borrowing in the next chapter.

    // Some types implment the Copy trait which means they are copied instead of moved.
    // All primitive types implement the Copy trait.
    let x = 7;
    let y = x;
    let z = y;
    println!("{} {} {}", x, y, z);

}
