extern crate serde;
extern crate serde_yaml;

use serde::{Serialize,Deserialize};
use std::fmt;


// #[derive(Debug)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Color  {
    RGB(u8,u8,u8),
    HEX(u8,u8,u8),
    HSL(u8,u8,u8),
    // maybe a later feature, need to research how rgba is stored 
    // RGBA(u8,u8,u8, f8),
}




impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RGB(r,g,b) => write!(f,"rgb({}, {}, {})", r,g,b),
            Color::HEX(r,g,b) => write!(f,"#{}{}{}", r,g,b),
            Color::HSL(r,g,b) => write!(f,"hsl({} {} {}) TODO", r,g,b),
        }
    }
}

