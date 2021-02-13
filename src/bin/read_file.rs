use std::fs::File;
use std::io::prelude::*;

pub fn get_input(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();
    data.trim().to_owned()
}