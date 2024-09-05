#![allow(unused)]

use std::{
    collections::{btree_map::Values, HashMap},
    env::args,
};

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
pub struct Parser {
    strings: HashMap<String, String>,
    ints: HashMap<String, i32>,
    bools: HashMap<String, bool>,
    doubles: HashMap<String, f64>,
    longs: HashMap<String, i64>,
    help_message: String,
}

impl Parser {
    pub fn new(exra_message: Option<&str>) -> Self {
        let mut instance = Self {
            strings: HashMap::new(),
            ints: HashMap::new(),
            bools: HashMap::new(),
            doubles: HashMap::new(),
            longs: HashMap::new(),
            help_message: String::new(),
        };
        let mut arg = args();

        instance.help_message += format!(
            "{}\t{}\n\nOptions:\n\n",
            arg.nth(0).unwrap(),
            exra_message.unwrap()
        )
        .as_str();

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

    pub fn parse(&mut self) {
        let mut value;
        let mut args_vec: Vec<String> = args().collect();

        for i in 0..args_vec.len() {
            value = args_vec[i].clone();
            if value.contains("--") {
                if self.bools.contains_key(&value[2..]) {
                    self.bools
                        .insert((&value[2..]).to_string(), args_vec[i + 1].parse().unwrap());
                }
                if self.doubles.contains_key(&value[2..]) {
                    self.doubles
                        .insert((&value[2..]).to_string(), args_vec[i + 1].parse().unwrap());
                }
                if self.ints.contains_key(&value[2..]) {
                    self.ints
                        .insert((&value[2..]).to_string(), args_vec[i + 1].parse().unwrap());
                }
                if self.strings.contains_key(&value[2..]) {
                    self.strings
                        .insert((&value[2..]).to_string(), args_vec[i + 1].parse().unwrap());
                }
                if self.longs.contains_key(&value[2..]) {
                    self.longs
                        .insert((&value[2..]).to_string(), args_vec[i + 1].parse().unwrap());
                }
            }
        }
    }

    pub fn get_int(&self, text: &str) -> i32 {
        self.ints.get(text).unwrap().clone()
    }

    pub fn get_bool(&self, text: &str) -> bool {
        self.bools.get(text).unwrap().clone()
    }

    pub fn get_string(&self, text: &str) -> String {
        self.strings.get(text).unwrap().clone()
    }

    pub fn get_double(&self, text: &str) -> f64 {
        self.doubles.get(text).unwrap().clone()
    }

    pub fn get_long(&self, text: &str) -> i64 {
        self.longs.get(text).unwrap().clone()
    }

    pub fn set_helpmessage(&mut self, message: &str) {
        self.help_message += format!("--help\t\t\t{}\n", message).as_str();
    }

    pub fn print_help(&mut self) {
        if !self.help_message.contains("--help") {
            self.set_helpmessage("Print help message");
        }

        println!("{}", self.help_message);
    }
}
