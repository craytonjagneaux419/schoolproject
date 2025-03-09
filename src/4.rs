use std::io;
use std::io::prelude::*;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    println!("You entered: {}", buffer.trim());
}
