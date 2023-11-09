use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use qolor::{palette::Palette, 
    ui::{start_terminal, 
        restore_terminal}
};
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

use toml::Table;


#[derive(Deserialize)]
struct Config {
    path: String,
    selected: String,
    auto_type: bool,
    close_on_select: bool,
    copy_to_clipboard: bool,
}

fn main() -> Result<()> {

    let pathstr = "./config.toml";

    let path = std::path::Path::new(pathstr);
    let file = std::fs::read_to_string(path)
        .expect("path to config not found");

    let config: Config = toml::from_str(&file).unwrap();

    let _ = start_terminal();
    loop {
        let catppuccin = Palette::load(&config.path, &config.selected)?;
        let input = qolor::ui::keyboard_selection(&catppuccin);

        //copied to clipboard
        let mut ctx = ClipboardContext::new().unwrap();

        
        if let Ok(keypress) = input {
            match keypress {
                Some(c) => { 
                    let to_copy = &catppuccin.get_string(c);
                    let _ = ctx.set_contents(to_copy.into());
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

