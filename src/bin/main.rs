use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::color;
use std::collections::HashMap;
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

    let c = "#0f00".parse::<Color>().unwrap();

    let mut cat = Palette::new("KittyColors"); 
    // println!("{}", c.to_hex_string());

    cat.add('q',
            "green",
            color::Format::HEX,
            &c.to_hex_string()
            ); 
    cat.add('w',
            "cyan",
            color::Format::HEXA,
            "00ffff"
            ); 
    cat.add('e',
            "magenta",
            color::Format::RGB,
            "magenta"
            ); 
    cat.add('r',
            "red",
            color::Format::RGBA,
            "ff000088"
            ); 
    cat.add('t',
            "glass",
            color::Format::RGBA,
            "transparent"
            ); 
    cat.save();


    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HEX  q: {}", loaded_cat.get_color('q').unwrap());
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HEXA w: {}", loaded_cat.get_color('w').unwrap());
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGB  e: {}", loaded_cat.get_color('e').unwrap());
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGBA r: {}", loaded_cat.get_color('r').unwrap());
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGBA t: {}", loaded_cat.get_color('t').unwrap());
    // let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    // println!("RGBA y: {}", loaded_cat.get_color('y').unwrap());
    

    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    let color = loaded_cat.get_color('q').unwrap();

    //copied to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(color).unwrap();

    //exit gui
}

