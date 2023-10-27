use std::env;
use std::fs;
use std::io::{self, BufRead};

pub fn find_color_by_key(key: char, file_path: &str) -> Option<String> {
    //could have used ? operator
    let contents = fs::read_to_string(file_path)
        .expect("Should have read the file"); 

    for line in contents.lines() {
        if line.starts_with(key) {
            return Some(line.to_string());
        }
    }
    return None;
}



