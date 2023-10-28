use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use std::collections::HashMap;
// use qolor::color::Color;
// use qolor::color_file_reader::find_color_by_key;
use std::fs;

use csscolorparser::Color;
use qolor::palette::Palette;



// opens menu
//
// displays all saved colors read from a file
//      literally plain text
//
// select color with keyboard
//
// each color mapped to different key
//      color correspnding is copied to the clipboard
//      exits when a key is pressed
fn main() {

    
    let mut cat = Palette::new("KittyColors"); 
    


    let c = "#0f00".parse::<Color>().unwrap();
    // println!("{}", c.to_hex_string());

    // cat.add('q', &c.to_hex_string()); 
    // cat.add('w', &c.to_hex_string()); 
    // cat.add('r', &c.to_hex_string()); 
    // cat.add('a', &c.to_hex_string()); 
    // cat.save();


    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();

    println!("{}", loaded_cat.get_color('q').unwrap());



    //copied to clipboard
    // let mut ctx = ClipboardContext::new().unwrap();
    // ctx.set_contents(color).unwrap();

    //exit gui
}

