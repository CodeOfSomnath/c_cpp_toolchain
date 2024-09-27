#![allow(unused)]

mod template;

use std::{fs::{self, File}, io::Write, os, path::{self, Path}};

pub fn create_new_file(file_path: &str) {
    File::create_new(file_path).unwrap();
}

pub fn create_new_folder(folder_path: &str) {
    fs::create_dir(folder_path).unwrap();
    
}

pub fn write_cmake_file(path: &str, project_name: &str, lang: template::Lang, line: &str) {
    let mut file = File::open(path).unwrap();
    file.write("cmake_minimum_required(VERSION 3.18)\n\n".as_bytes());
    file.write(format!("project({})\n\n", project_name).as_bytes());
    file.write(line.as_bytes());
    file.flush().unwrap()
}


