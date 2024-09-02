#![allow(unused)]

use std::{collections::HashMap, env::args};

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
    strings: HashMap<String, String>,
    ints: HashMap<String, i32>,
    bools: HashMap<String, bool>,
    doubles: HashMap<String, f64>,
    longs: HashMap<String, i64>,
    help_message: String 
}

impl Parser {
    pub fn new(exra_message: Option<&str>) -> Self {
        let mut instance = Self {
            strings: HashMap::new(),
            ints: HashMap::new(),
            bools: HashMap::new(),
            doubles: HashMap::new(),
            longs: HashMap::new(),
            help_message: String::new()
        };
        let mut arg = args();
        
        instance.help_message += format!("{} {}", arg.nth(0).unwrap(), exra_message.unwrap()).as_str();

        return instance;
    }

    pub fn add_int(&mut self, text: &str, help: &str, default: i32) {
        self.ints.insert(text.to_string(), default);
        let message = format!("--{}\t\t\t{}\n", text, help);
        self.help_message += message.as_str();

    }

    pub fn add_bool(&mut self, text: &str, help: &str, default: bool) {
        self.bools.insert(text.to_string(), default);
        let message = format!("--{}\t\t\t{}\n", text, help);
        self.help_message += message.as_str();
    }

    pub fn add_string(&mut self, text: &str, help: &str, default: &str) {
        self.strings.insert(text.to_string(), default.to_string());
        let message = format!("--{}\t\t\t{}\n", text, help);
        self.help_message += message.as_str();
    }

    pub fn add_double(&mut self, text: &str, help: &str, default: f64) {
        self.doubles.insert(text.to_string(), default);
        let message = format!("--{}\t\t\t{}\n", text, help);
        self.help_message += message.as_str();
    }

    pub fn add_long(&mut self, text: &str, help: &str, default: i64) {
        self.longs.insert(text.to_string(), default);
        let message = format!("--{}\t\t\t{}\n", text, help);
        self.help_message += message.as_str();
    }

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
