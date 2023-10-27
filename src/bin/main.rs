use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use std::collections::HashMap;
use qolor::color::Color;
use qolor::color_file_reader::find_color_by_key;
use std::fs;



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
    //display all saved colors
    // let mut mypalette = qolor::palette::Palette::new("Catppuccin");
    // mypalette.colors.insert('q', Color::HEX(17, 17, 27));
    // mypalette.colors.insert('w', Color::RGB(0, 17, 27));
    // mypalette.colors.insert('e', Color::HSL(17, 17, 27));
    //
    //
    // // let yaml = serde_yaml::to_string(&mypalette).unwrap();
    //
    // // println!("{}", yaml);
    //
    // let _ = mypalette.save();
    //
    //
    // let newpal = qolor::palette::Palette::new("This filename /isn't .?!.. safe");
    //
    // let _ = newpal.save();

    
    let cat = qolor::palette::Palette::load("Catppuccin").expect("loading");
    println!("this is what I've got :{}",cat.colors.get(&'q').unwrap());


        


    // finds the selected color
    // let keyboardpress: &char = &'w';
    // let color = colors.get(keyboardpress).unwrap();
    // let search = find_color_by_key('q', "../colors.txt") ;
    // match search {
    //     Some(c) => color = c,
    //     None => panic!("unknown"),
    // }

    
    
    // println!("{color}");


    //copied to clipboard
    // let mut ctx = ClipboardContext::new().unwrap();
    // ctx.set_contents(color).unwrap();

    //exit gui
}

