use ferris_says::say;
use std::io::{stdout, BufWriter};

// Level 0 - Hello World 
fn hello_world() {
    println!("Hello, World!");
}

// Level 1 - Hello Universe
fn hello_universe() {
    let stdout = stdout();
    let message = String::from("Hello, Universe!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn main() {
    hello_world();
    hello_universe();
}