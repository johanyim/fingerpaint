use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file could not be found at {0}.")]
    NotFound(String),
    #[error("File at {0} was not a valid .toml configuration.")]
    CouldNotParse(#[from] toml::de::Error),
    #[error("Config folder (${0}) could not be found.")]
    NoConfigFolder(String),
    #[error("No configuration was used.")]
    NoConfiguration,
}


#[derive(Error, Debug)]
pub enum PaletteError {
    #[error("The path to the palettes directory is 'None'. Cannot determine where to save this file.")]
    PalettesPathNotFound,
    #[error("Could not write to file at {1}. {0}")]
    CouldNotWrite(std::io::Error, String),
    #[error("File at {1} could not be created. {0}")]
    CouldNotCreate(std::io::Error, String),
    #[error("Could not find the file at {1}. {0}")]
    CouldNotFind(std::io::Error, String),
    #[error("File at {1} was not a valid .yaml palette. {0}")]
    CouldNotParse(serde_yaml::Error, String),
}
