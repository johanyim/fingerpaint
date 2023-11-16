use crate::error::ConfigError::{
    self, CouldNotParse, NotFound, NoConfigFolder, NoConfiguration};
use casual::confirm;

use std::{path::Path, fs};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// path to palettes directory, Use default config is set to none
    pub palettes: Option<String>,
    pub selected: Option<String>,
    pub auto_type: bool,
    pub copy_to_clipboard: bool,
    pub close_on_select: bool,
    pub stdout: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config{
            // palettes: Some("/home/johan/.config/fingerpaint/palettes".to_string()),
            palettes: None,  
            selected: None,
            auto_type: true,
            close_on_select: true,
            copy_to_clipboard: false,
            stdout: true,
        }
    }
}

impl Config {
    pub fn build(config_arg: Option<String>) -> Result<Self, ConfigError> {
        
        let readfile = match config_arg {
            Some(config_path) => read_specified_path(config_path),
            None => read_default_path(),
        };

        let contents: String = match readfile {
            Ok(c) => c,
            Err(_) => return prompt_use_default(),
        };

        match parse_contents(contents) {
            Ok(config) => return Ok(config),
            Err(_) => return prompt_use_default(),
        }
    }
}

fn read_specified_path(pathstr: String) -> Result<String, ConfigError>{
    let path = Path::new(&pathstr);
    match fs::read_to_string(path) {
        Err(_) => {
            eprintln!("Could not read the specified path: {pathstr}");
            return Err(NotFound(pathstr))
        },
        Ok(contents) => return Ok(contents),
    };
}

fn read_default_path() -> Result<String, ConfigError> {
    let pathstr: String = get_default_path()?;    
    let path = Path::new(&pathstr);
    match fs::read_to_string(path) {
        Err(_) => {
            eprintln!("Could not read the default path: {pathstr}");
            // if confirm("Would you like to make a default configuration at {pathstr}?") {
            //     todo!("Makes the config")
            prompt_create(pathstr.clone())?;
            return Ok(pathstr);
        }
        Ok(contents) => return Ok(contents),
    };
} // -c is unspecified use system default (created or not created)

fn get_default_path() -> Result<String, ConfigError> {
    //look for a configuration file 
    //read from environment, or give a default
    //Look for configuration location environment variable if specified
    let config_home_str = "XDG_CONFIG_HOME";
    let config_home = match std::env::var(config_home_str) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Configuration home path (${config_home_str}) is not set, default file could not be found.");
            return Err(NoConfigFolder(config_home_str
                           .to_string())
                       )
        },
    };
    //XDG_CONFIG_HOME/fingerpaint/fingerpaint.toml
    let mut conf_path = std::path::PathBuf::new();
    conf_path.push(config_home);
    conf_path.push("fingerpaint");
    conf_path.push("fingerpaint");
    conf_path.set_extension("toml");

    Ok(conf_path.into_os_string()
       .to_str()
       .unwrap_or("config home string config")
       .to_string())
}


fn parse_contents(contents: String) -> Result<Config, ConfigError> {
    return match toml::from_str(&contents) {
        Ok(config) => Ok(config),  
        Err(e) => {
            eprintln!("{e} Could not parse toml file");
            return Err(CouldNotParse(e))
        },
    };
}

fn prompt_create(pathstr: String) -> Result<(), ConfigError> {
    if confirm(format!("Would you like to create a default configuration file at {pathstr}?")){
        let config = Config::default();
        let toml_string = toml::to_string(&config)
            .expect("Could not encode TOML value ");
        fs::write(pathstr, toml_string)
            .expect("Could not write to file");
        return Ok(());
    } else {
        return Err(NoConfiguration);
    }
} 

fn prompt_use_default() -> Result<Config, ConfigError>{
    if confirm("Run with default settings?") {
        return Ok(Config::default())
    }else{
        return Err(NoConfiguration)
    }
}

