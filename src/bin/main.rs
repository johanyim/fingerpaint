use std::{path::Path, result};

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::{palette::Palette, 
    ui::{start_terminal, 
        restore_terminal},
    config::Config,
};
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

use toml::Table;
use clap::{Parser, Subcommand};




#[derive(Subcommand)]
enum Command {
    /// Set a color given key and color for selected palette
    Set { key: char , color: Option<String>},
    /// Removes color at the key specified for the selected palette
    Remove { key: char },

    New { name: Option<String> },

}

#[derive(Parser)]
// #[command(name = "FingerPaint")]
// #[command(author = "Johan Y. <johanjyyim@gmail.com>")]
// #[command(version = "1.0")]
// #[command(about = "Easy color selection", long_about = None)]
#[command(author, version, about, long_about)]
struct Arguments {
    #[command(subcommand)]
    subcommand: Option<Command>,
    #[arg(short, long)]
    palette: Option<String>,
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {

    let args = Arguments::parse();

    let c_arg: Option<String>= args.config;

    let config: Config = Config::build(c_arg)?;
    //loading palette
    let mut palette = Palette::load(&config)?;

    if let Some(subcommand) = args.subcommand {
        match subcommand {
            Command::Set { key, color} => {
                set(&config, &mut palette, key, color);
                return Ok(())
            },
            Command::Remove { key } => { 
                remove(&config, &mut palette, key);
                return Ok(())
            },
            Command::New { name } => { 
                todo!();
                return Ok(())
            },
        }
    } else {

    }


    let mut terminal = start_terminal()?;
    loop {
        let input = qolor::ui::keyboard_selection(&mut terminal, &palette);

        //copied to clipboard
        let mut ctx = ClipboardContext::new().unwrap();
        
        if let Ok(keypress) = input {
            match keypress {
                Some(c) => { 
                    let to_copy = &palette.get_string(c);
                    if config.copy_to_clipboard  {
                        let _ = ctx.set_contents(to_copy.into());
                    }
                },
                None => break,
            } 
        }

        if config.close_on_select {
            break
        }
    }

    restore_terminal()?;
    //exit gui
    Ok(())
}




// pub fn run(config: &Config, )


// set <KEY> [color]
pub fn set(config: &Config, palette: &mut Palette, key: char, color: Option<String>) {

    let mut ctx = ClipboardContext::new().unwrap();

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
    palette.set(key, "Unnamed", qolor::color::Format::HEX, &valid_color)
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

