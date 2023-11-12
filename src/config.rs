#[derive(serde::Deserialize)]
pub struct Config {
    pub palettes: String,
    pub selected: String,
    pub auto_type: bool,
    pub copy_to_clipboard: bool,
    pub close_on_select: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config{
            palettes: "/home/johan/.config/fingerpaint/palettes".to_string(),
            selected: "catppuccin".to_string(),
            auto_type: true,
            close_on_select: true,
            copy_to_clipboard: false,
        }
    }
}

fn get_default_path() -> anyhow::Result<OsString> {
    //look for a configuration file 
    //read from environment, or give a default
    //Look for configuration location environment variable if specified
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .expect("user has a config home path");
    let mut conf_path = std::path::PathBuf::new();
    conf_path.push(config_home);
    conf_path.push("fingerpaint");
    conf_path.push("fingerpaint");
    conf_path.set_extension("toml");

    Ok(conf_path.into_os_string())
}
use std::{path::Path, ffi::OsString};
impl Config {

    fn specified() {todo!()} // -c is specified (valid or invalid)
    fn unspecified() {todo!()} // -c is unspecified use system default (created or not created)
    fn prompt_create() {todo!()} // prompts user to create a configuration at the location
                            // error



    pub fn build(config_arg: Option<String>) -> anyhow::Result<Self> {
        match config_arg {
            None => {
                
                let config_osstring: OsString = match get_default_path() {
                    Ok(path) => path,
                    Err(e) => return Err(e),
                };
                let config_path: &Path = Path::new(&config_osstring);

                if config_path.exists() {
                    //read the config, and return it
                    return Ok(toml::from_str(
                            &std::fs::read_to_string(config_path)
                            .expect("Default path should have been valid"))
                        .expect("Default config should have been readable as .toml"));
                }else{
                    //prompt
                    if casual::confirm( 
                        format!("The config at \"{}\" could not be found, \
                                would you like to make one?", 
                                config_path.to_str().unwrap())) 
                    {
                        todo!("touch config_location, write default config to default location, return config_config;");
                    }else {
                        println!("Using default configuration.");
                        return Ok(Config::default());
                    }
                }
            },
            Some(pathstr) => {
                let path = std::path::Path::new(&pathstr);

                if path.exists() {
                    //read the config, and return it
                    match std::fs::read_to_string(path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("\"{pathstr}\" was not a valid configuration path."); 
                            if casual::confirm("Use default configuration instead?") {
                                return Ok(Config::default());
                            }else{
                                return Err(e.into());
                            };
                        },
                    };

                }else{
                    eprintln!("No file found at {pathstr}.");
                    if casual::confirm("Use default configuration instead?") {
                        return Ok(Config::default());
                    }else{
                        todo!();
                    };
                }

                let file = std::fs::read_to_string(path)
                    .expect("path to config not found");

                match toml::from_str(&file) {
                    Ok(config) => return Ok(config),  
                    Err(e) =>{ 
                        eprintln!("\"{pathstr}\" could not be parsed as a .toml file");
                        if casual::confirm("Use default configuration instead?") {
                            return Ok(Config::default());
                        }else{
                            return Err(e.into());
                        };
                    }
                }
            },
        }
    }
}





