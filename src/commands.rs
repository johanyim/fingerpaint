use crate::{
    palette::Palette, 
    config::Config,
    color::Format,
};
use crate::ui::{
    start_terminal, restore_terminal, 
    keyboard_selection
};
use anyhow::Result;
use clap::Subcommand;
// use copypasta::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider} ;

#[derive(Subcommand)]
pub enum Command {
    /// Set a color given key and color for selected palette
    Set { key: char , color: Option<String>},
    /// Removes color at the key specified for the selected palette
    Remove { key: char },

    New { name: Option<String> },

}


// set <KEY> [color]
pub fn set(config: &Config, palette: &mut Palette, key: char, color: Option<String>) {

    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();

    //either read clipboard (hopefully a color)
    //or read the argument passed
    let contents = match color {
        None => ctx.get_contents()
            .expect("Clipboard content should be obtainable"),
        Some(string) => string,
    };                

    //parsing the color
    let valid_color = match csscolorparser::parse(&contents) {
        Err(e) => {
            eprintln!("\"{contents}\" could not be parsed as a valid color. Error: {e}");
            return;
        },
        Ok(color) =>  color.to_hex_string() ,
    };

    // at this point, we must have a valid color
    palette.set(key, "Unnamed", Format::HEX, &valid_color)
        .expect("Should have been a valid color from contents");

    let _ = palette.save(&config);
    println!("\"{valid_color}\" was saved to key {key} as {}.", "Unnamed");
}

// remove <KEY>
pub fn remove(config: &Config, palette: &mut Palette, key: char){
    match palette.remove(key) {
        Some(color) => println!("{} was removed from key {}.", color.name, key),
        None => println!("No color was found at key {key}"),
    }
    let _ = palette.save(&config);
}


use crate::error::PaletteError;
pub fn new(config: &Config, name: Option<String> ) -> Result<(), PaletteError> {

    let palettename = match name { 
        None => casual::prompt("Name: ").default("Unnamed".to_string()).get(),
        Some(n) => n 
    };
    let mut palette = Palette::new(&palettename);
    let _ = run(&config, &mut palette);
    palette.save(config)?;
    Ok(())

}
// use copypasta::prelude::*;
use std::process::{self, Stdio};
use crate::ui;
pub fn run(config: &Config, palette: &mut Palette) -> Result<()> {
    let mut terminal = ui::start_terminal()?;
    loop {
        let input = ui::keyboard_selection(&mut terminal, &palette);

        //copied to clipboard
        let mut ctx = ClipboardContext::new().unwrap();

        if let Ok(keypress) = input {
            match keypress {
                Some(c) => { 
                    let to_copy = &palette.get_string(c);
                    if config.copy_to_clipboard  {
                        ctx.set_contents(to_copy.to_owned())
                            .expect("should have copied to clipboard");
                    }
                },
                None => break,
            } 
        }
        if config.close_on_select {
            break
        }

    }

    ui::restore_terminal()?;
    Ok(())
}
