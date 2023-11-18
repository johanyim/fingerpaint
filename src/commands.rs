use crate::{
    palette::Palette, 
    config::Config,
    color::Format,
    ui,
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
    let mut palettes = vec![Palette::new(&palettename)];
    let _ = run(&config, &mut palettes);
    palettes[1].save(config)?;
    Ok(())

}
// use copypasta::prelude::*;
pub fn run(config: &Config, palettes: &mut Vec<Palette>) -> Result<()> {
    let mut terminal = ui::start_terminal()?;
    let mut index = 1; //todo can define the index in config
    loop {
        let input = ui::color_select(&mut terminal, palettes, &mut index);

        //copied to clipboard
        let mut ctx = ClipboardContext::new().unwrap();

        if let Ok(keypress) = input {
            match keypress {
                Some(c) => { 
                    let to_copy = &palettes[index].get_string(c);
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

