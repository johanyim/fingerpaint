extern crate serde;
extern crate serde_yaml;

use core::fmt;
use serde::{Serialize,Deserialize};
use csscolorparser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Color{
    pub name: String, 
    pub output_format: Format,
    pub hex_color: String, 
}

impl Color {
    pub fn new(name: &str, color: &str) -> Self{
        
        return Color{
            name: name.to_string(),
            output_format: Format::HEX,
            hex_color: csscolorparser::parse(color)
                .expect("couldn't parse color")
                .to_hex_string(), 

        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    RGB, HEX, HSL,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.output_format {
            Format::HEX => write!(f, "{}", self.hex_color),
            Format::RGB => write!(f, "{}", 
                                  csscolorparser::parse(&self.hex_color)
                                  .unwrap()
                                  .to_rgb_string() 
                                  ),
            _ => write!(f, "other format"),
        }
    }
}





// impl Color {
//     //conversion
//     pub fn convert_to(self, f: Format) -> Color {
//         let (r,g,b) = match self {
//             Color::RGB(r,g,b) => (r,g,b),
//             Color::HEX(r,g,b) => (r,g,b),
//             Color::HSL(r,g,b) => (r,g,b),
//         };
//         return match f {
//             Format::RGB => Color::RGB(r,g,b),
//             Format::HEX => Color::HEX(r,g,b),
//             Format::HSL => Color::HSL(r,g,b),
//         }
//     }
//     // RGB(1,1,1).convert_to(RGB)
// }
//
// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Color::RGB(r,g,b) => write!(f,"rgb({},{},{})", r,g,b),
//             Color::HEX(r,g,b) => write!(f,"#{:02x}{:02x}{:02x}", r,g,b),
//             Color::HSL(r,g,b) => write!(f,"hsl({} {} {}) TODO", r,g,b),
//         }
//     }
// }
//




