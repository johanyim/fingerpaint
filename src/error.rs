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
