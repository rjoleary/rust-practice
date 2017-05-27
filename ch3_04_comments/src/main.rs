//! Second style of doc comment.
//!
//! This documents the containing item, for example the containing crate.

// Line comments: not much to say about this...

/// Doc comment for main.
///
/// # Examples
/// ```
/// cargo run
/// ```
///
/// These doc comments are written with Markdown. Use the rustdoc tool to generate documentation
/// from these comments.
fn main() {
    println!("Hello, world!");
}
