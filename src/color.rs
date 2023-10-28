extern crate serde;
extern crate serde_yaml;

use core::fmt;
use serde::{Serialize,Deserialize};
use csscolorparser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Color{
    pub name: String, 
    pub output_format: Format,
    pub rgba_color: [u8; 4], 
}

impl Color {
    pub fn new(name: &str, format: Format, color: &str) -> Self{
        
        return Color{
            name: name.to_string(),
            output_format: format,
            rgba_color: csscolorparser::parse(color)
                .expect("couldn't parse color")
                .to_rgba8(), 

        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    RGB,RGBA,HEX,HEXA,HSL,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.output_format {
            Format::RGB => write!(f, "rgb({},{},{})", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2]),
            Format::RGBA => write!(f, "rgba({},{},{},{})", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2], self.rgba_color[3]),
            Format::HEX => write!(f, "#{:02x}{:02x}{:02x}", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2]),
            Format::HEXA => write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2], self.rgba_color[3]),
            _ => write!(f, "Format unknown, please check format of color"),
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




