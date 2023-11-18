mod color;
mod commands;
mod config;
mod error;
mod palette;
mod ui;

use palette::Palette;
use config::Config;
use commands::*;

use anyhow::Result;
use clap::Parser;
use ui::tabline::Tabline;


#[derive(Parser)]
// #[command(name = "FingerPaint")]
// #[command(author = "Johan Y. <johanjyyim@gmail.com>")]
// #[command(version = "1.0")]
// #[command(about = "Easy color selection", long_about = None)]
#[command(author, version, about, long_about)]
struct Arguments {
    #[command(subcommand)]
    subcommand: Option<Command>,
    ///Palette to use upon startup
    #[arg(short, long)]
    palette: Option<String>,
    ///Path to configuration file
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {

    let args = Arguments::parse();

    let config_arg: Option<String>= args.config;

    let config: Config = Config::build(config_arg)?;
    //loading palette
    // let mut palette = Palette::load(&config)?;
    let mut palettes = Palette::load_all(&config)?; 

    let index = 1;
    match args.subcommand {
        Some(Command::Set{key,color}) => { 
            set(&config, &mut palettes[index], key, color); },
        Some(Command::Remove{key}) => { 
            remove(&config, &mut palettes[index], key); },
        Some(Command::New{name}) => { 
            new(&config, name)?; },
        None => run(&config, &mut palettes)?,
    }
    //exit gui
    Ok(())
}

