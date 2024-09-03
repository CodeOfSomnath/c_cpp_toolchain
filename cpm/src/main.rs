// This is main package manager binarary for distribution

use std::env::args;

use argparser;

fn main() {
    let mut parser = argparser::Parser::new(Some("Testing project"));
    parser.add_int("big", "This is a help", 0);
    parser.parse();
    let value = parser.get_int("big");
    println!("{}", value);
}
