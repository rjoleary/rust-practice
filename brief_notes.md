
Installation:

    $ curl https://sh.rustup.rs -sSf | sh

Cargo tool:

    $ cargo build
    $ cargo run
    $ cargo new hello_world --bin

Hello world:

    fn main() {
        println!("Hello, world!");
    }

Conventions:
    filenames contain "_"

Shadowing: within the same scope, a very may be declared with the same name as
           a previous variable in the scope

Macros: functions ending in a `!` such as `println!`

Immutable: the default, use `mut` keyword for mutable

Bindings for variables.

`println!` uses `{}` for string interpolation.
