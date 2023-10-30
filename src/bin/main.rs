use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::color;
// use std::collections::HashMap;
// use std::fs;

use csscolorparser::Color;
use qolor::palette::Palette;


//
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
    cat.add('t',
            "glass",
            color::Format::RGBA,
            "transpa"
            ); 
    cat.add('y',
            "navy",
            color::Format::CMYK,
            "#2D4192"
            ); 
    cat.add('y',
            "navy",
            color::Format::HSL,
            "rgb(100,21,12)"
            ); 
    cat.add('u',
            "transparent navy",
            color::Format::HSL,
            "rgb(100,21,12,0.1)"
            ); 
    cat.add('a', 
                "opaque green",
                color::Format::HSL,
                "#00ff00ff"
               ); 

    cat.add('s', 
                "half transparent blue",
                color::Format::HSL,
                "#0000ff80"
               ); 

    cat.add('d', 
                "opaque white",
                color::Format::HSL,
                "rgb(255,255,255)"
               ); 

    cat.add('f', 
                "opaque grey",
                color::Format::HSL,
                "rgb(128,128,128)"
               ); 

    cat.add('g', 
                "opaque black",
                color::Format::HSL,
                "rgb(0,0,0)"
               ); 

    let _ =  cat.save();


    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HEX  q: {}", loaded_cat.get_string('q'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HEXA w: {}", loaded_cat.get_string('w'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGB  e: {}", loaded_cat.get_string('e'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGBA r: {}", loaded_cat.get_string('r'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("RGBA t: {}", loaded_cat.get_string('t'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("CMYK y: {}", loaded_cat.get_string('y'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("CMYK u: {}", loaded_cat.get_string('u'));
    
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HSL a: {}", loaded_cat.get_string('a'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HSL s: {}", loaded_cat.get_string('s'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HSL d: {}", loaded_cat.get_string('d'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HSL f: {}", loaded_cat.get_string('f'));
    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    println!("HSL g: {}", loaded_cat.get_string('g'));

    

    let loaded_cat: Palette = Palette::load("KittyColors").unwrap();
    let color = loaded_cat.get_string('q');

    //copied to clipboard
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(color).unwrap();

    //exit gui
}
