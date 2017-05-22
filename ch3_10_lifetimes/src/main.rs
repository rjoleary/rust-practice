// Lifetime syntax for functions
fn skip_prefix<'a>(line: &'a str, prefix: &str) -> &'a str {
    if line.len() < prefix.len() || &line[..prefix.len()] != prefix {
        return line
    }
    return &line[prefix.len()..]
}

// Lifetimes are required for structs containing a reference.
struct Foo<'a> {
    x: &'a i32,
}

// If you return a reference, and there are no
fn my_name() -> &'static str {
    "Ryan"
}

fn main() {
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);
        v = skip_prefix(line, p.as_str());
    }
    println!("{}", v);

    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x);

    // The static lifetime indicates the refences has a lifetime of the entire program.
    let x: &'static str = "Hello, world.";
    println!("{}", x);

    // Another example:
    static Z: i32 = 5;
    let x: &'static i32 = &Z;
    println!("{}", x);

    println!("{}", my_name());
}
