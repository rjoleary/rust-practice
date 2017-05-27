fn main() {
    // While loop
    let mut i = 1;
    while i < 1000 {
        i *= 2;
        println!("{}", i);
    }

    // For loop
    // Right side must be convertable to iterator via `IntoIterator`
    for v in 0..10 { // does not include 10
        println!("{}", v);
    }
    println!("(1..10).len() == {}", (1..10).len());
    for (i, v) in (0..10).enumerate() {
        println!("{}: {}", i, v);
    }

    // Infinite loop
    let mut counter = 0;
    loop {
        println!("Loop foreverish!");
        counter += 1;
        if counter == 5 {
            break; // exits early
            // The "continue" keyword is also supported
        }
    }

    // Loop labels
    'outer: for x in 0..10 {
		'inner: for y in 0..10 {
			if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
			if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
			println!("x: {}, y: {}", x, y);
		}
	}
}
