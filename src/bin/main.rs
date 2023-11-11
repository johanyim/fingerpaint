use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::{palette::Palette, 
    ui::{start_terminal, 
        restore_terminal}
};
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

use toml::Table;
use clap::{Parser, Subcommand};


#[derive(Deserialize)]
struct Config {
    path: String,
    selected: String,
    auto_type: bool,
    close_on_select: bool,
    copy_to_clipboard: bool,
}


#[derive(Subcommand)]
enum Command {
    /// Set a color given key and color for selected palette
    Set { key: char , color: Option<String>},
    /// Removes color at the key specified for the selected palette
    Remove { key: char }

}

#[derive(Parser)]
// #[command(name = "FingerPaint")]
// #[command(author = "Johan Y. <johanjyyim@gmail.com>")]
// #[command(version = "1.0")]
// #[command(about = "Easy color selection", long_about = None)]
#[command(author, version, about, long_about)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(short, long)]
    palette: Option<String>,
    #[arg(short, long)]
    config: Option<String>,


}

fn main() -> Result<()> {

    let args = Arguments::parse();
    let pathstr = match args.config {
        Some(p) => p.to_string(),
        None => String::from("./config.toml"),
    };

    let path = std::path::Path::new(&pathstr);
    let file = std::fs::read_to_string(path)
        .expect("path to config not found");

    let config: Config = toml::from_str(&file).unwrap();

    let mut palette = Palette::load(&config.path, &config.selected)?;

    // palette.set('1', "testing", qolor::color::Format::RGBA, "#000444")?;

    if let Some(command) = args.command {
        match command {
            Command::Set { key, color} => {
                let mut ctx = ClipboardContext::new().unwrap();
                let contents = match color {
                    //no arguments after 'set' = read clipboard
                    None => ctx.get_contents()
                        .expect("Clipboard content should be obtainable"),

                    //some argument after 'set' = color has been specified 
                    Some(string) => {
                        match csscolorparser::parse(&string) {
                            Ok(color) =>  color.to_hex_string() ,
                            Err(e) => {eprintln!("\"{string}\" could not be parsed as a valid color. Error: {e}");
                            return Ok(())},
                        }
                    }
                        // .expect(&format!("{} is not a valid color", &string))},
                };                
                match palette.set(key, "Unnamed", qolor::color::Format::HEX, &contents) {
                    Ok(_) => println!("\"{}\" was saved to key {} as {contents}.", "Unnamed", key),
                    Err(e) => println!("\"{contents}\" could not be parsed as a color. Error: {e}"),
                } 
                let _ = palette.save(&config.path);
                return Ok(())
            },
            Command::Remove { key } => { 
                match palette.remove(key) {
                    Some(color) => println!("{} was removed from key {}.", color.name, key),
                    None => println!("No color was found at key {key}"),
                }

                let _ = palette.save(&config.path);
                return Ok(())
            },

        }

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

    let _ = restore_terminal();
    //exit gui
    Ok(())
}

