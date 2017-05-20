fn main() {
    let x = 5;

    // Use like a statement
    if x == 3 {
        println!("x == 3");
    } else if x == 4 {
        println!("x == 4");
    } else if x == 5 {
        println!("x == 5");
    } else {
        println!("x < 3 or 5 < x");
    }

    // Use an expression
    let y = -4;
    let abs = if y < 0 { -y } else { -y };
    println!("abs of {} is {}", y, abs);
}
