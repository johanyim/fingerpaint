use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

use std::env;
use std::fs;
use std::io::{self, BufRead};


// opens menu
// displays all saved colors read from a file
//      literally plain text
// select color with keyboard
// each color mapped to different key
//   exits when a key is pressed
// color correspnding is copied to the clipboard
fn main() {
    let search = find_color_by_key('q', "colors.txt") ;
    let color: String;
    match search {
        Some(c) => color = c,
        None => panic!("unknown"),
    }
    println!("my color is {}", color);

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents("my string".into()).unwrap();
    println!("clipboard after : {:?}", ctx.get_contents());
}


fn find_color_by_key(key: char, file_path: &str) -> Option<String> {
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
