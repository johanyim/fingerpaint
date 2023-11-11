pub mod palette;
pub mod color;
pub mod ui;
pub mod cl_args;
pub mod config {
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
    
    impl Config {
        pub fn read(config_path: Option<String>) -> anyhow::Result<Self> {
            match config_path {
                None => {
                    //look for a configuration file 
                    let default_location = "/home/johan/.config/fingerpaint/config.toml"; //read from environment, or give a default
                    let default_path = std::path::Path::new(default_location);

                    if default_path.exists() {
                        //read the config, and return it
                        return Ok(toml::from_str(
                                &std::fs::read_to_string(default_path)
                                .expect("Default path should have been valid"))
                            .expect("Default config should have been readable as .toml"));
                    }else{
                        //prompt
                        if casual::confirm( 
                            format!("The config at \"{}\" could not be found, \
                                    would you like to make one?", 
                                    default_location)) 
                        {
                            todo!("
                                  //touch default_location,
                                  //write default config to default location
                                  //return default_config;
                                  ");
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
}



