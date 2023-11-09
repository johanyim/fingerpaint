extern crate serde;
extern crate serde_yaml;

use core::fmt;
use serde::{Serialize,Deserialize};
use csscolorparser::{self, ParseColorError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Color{
    pub name: String, 
    pub output_format: Format,
    pub rgba_color: [u8; 4], 
}

impl Color {
    pub fn new(name: &str, format: Format, color: &str) -> Result<Self, ParseColorError>{
        return Ok(Color{
            name: name.to_string(),
            output_format: format,
            rgba_color: csscolorparser::parse(color)?
                .to_rgba8(), 
        })
    }
    
    pub fn to_cmyk_string(&self) -> String {
        
        let [r8,g8,b8,_] = self.rgba_color;

        let r = (r8 as f32) / 255f32;
        let g = (g8 as f32) / 255f32;
        let b = (b8 as f32) / 255f32;

        let k: f32 = ((255 - std::cmp::max(std::cmp::max(r8,g8),b8)) as f32) / 255f32;
        let c: f32 = 100.0 * (1.0-r-k) / (1.0-k);
        let m: f32 = 100.0 * (1.0-g-k) / (1.0-k);
        let y: f32 = 100.0 * (1.0-b-k) / (1.0-k);

        return format!("cmyk({:.0}%, {:.0}%, {:.0}%, {:.0}%)", 
                       c.abs(),
                       m.abs(),
                       y.abs(),
                       k.abs()*100.0);
    }

    pub fn to_hsl_string(&self) -> String {
        let [r8,g8,b8,a8] = self.rgba_color;

        let r = (r8 as f32) / 255f32;
        let g = (g8 as f32) / 255f32;
        let b = (b8 as f32) / 255f32;


        let c_max: f32 = (std::cmp::max(std::cmp::max(r8,g8),b8) as f32) / 255f32;
        let c_min: f32 = (std::cmp::min(std::cmp::min(r8,g8),b8) as f32) / 255f32;

        let delta = (c_max - c_min).abs();
        
        let hue = 
            //roughly 0deg
            if delta <= 0.001 { 0.0 }
            //r is max
            else if r >= g && r >= b { 60.0*(((g-b)/delta)%6.0) }  
            //g is max
            else if g >= b { 60.0*(((b-r)/delta)+2.0) }  
            //b is max
            else{ 60.0*(((r-g)/delta)+4.0) } ;

        let light = (c_max + c_min) / 2.0;

        let saturation = 
            if delta <= 0.001 { 0.0 }
            else {
                delta / (1.0 - (2.0*light - 1.0).abs())
            };

        let alpha = 
            if a8 < 255 { format!(" / {:.2}", ((a8 as f32) /255.0) )}
            else { "".to_string() };

        return format!("hsl({:.0}deg {:.0}% {:.0}%{})", 
                       hue.abs(),
                       saturation.abs()*100.0,
                       light.abs()*100.0,
                       alpha);
    }


}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    RGB,RGBA,HEX,HEXA,CMYK,HSL,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.output_format {
            Format::RGB => write!(f, "rgb({},{},{})", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2]),
            Format::RGBA => write!(f, "rgba({},{},{},{})", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2], self.rgba_color[3]),
            Format::HEX => write!(f, "#{:02x}{:02x}{:02x}", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2]),
            Format::HEXA => write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.rgba_color[0], self.rgba_color[1], self.rgba_color[2], self.rgba_color[3]),
            Format::CMYK => write!(f, "{}", self.to_cmyk_string()),
            Format::HSL => write!(f, "{}", self.to_hsl_string()),
            _ => write!(f, "Format unknown, please check format of color"),
        }
    }
}






