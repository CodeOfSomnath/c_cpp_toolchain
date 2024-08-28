#![allow(unused)]

use std::collections::HashMap;

// creating a basic structure for relate functions
/// This is the structure to store all the values
/// ### Example
/// ```
/// let parser = Parser::new();
/// parser.add_int(
///     "number",
///     "This is a sample random number",
///     true,
///     10
/// );
/// parser.parse();
/// let value = parser.get_int();
/// println!("{}", value);
/// ```
struct Parser {
    values: HashMap<String, String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn add_int(text: &str, help: &str, required: bool, default: i32) {}

    pub fn add_bool(text: &str, help: &str, required: bool, default: bool) {}

    pub fn add_string(text: &str, help: &str, required: bool, default: &str) {}

    pub fn add_double(text: &str, help: &str, required: bool, default: f64) {}

    pub fn add_long(text: &str, help: &str, required: bool, default: i64) {}

    pub fn parse() {}

    pub fn get_int() -> i32 {
        0
    }

    pub fn get_bool() -> bool {
        false
    }

    pub fn get_string() -> String {
        "".to_string()
    }

    pub fn get_double() -> f64 {
        0.0
    }

    pub fn get_long() -> i64 {
        0
    }
}
