use csscolorparser::{self,Color};
use qolor::palette::Palette;
use qolor::color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_colors() {
        let mut testpal = Palette::new("testpal");
        let black = "000000";

        let _ = testpal.add('q', "green", color::Format::HEX, "rgb(0,255,0)"); 
        let _ = testpal.add('w', "cyan", color::Format::HEXA, "00ffff"); 
        let _ = testpal.add('e', "magenta", color::Format::RGB, "magenta"); 
        let _ = testpal.add('r', "red", color::Format::RGBA, "ff000088"); 
        let _ = testpal.add('t', "glass", color::Format::RGBA, "transparent"); 
        let _ = testpal.add('y', "black", color::Format::HEX, black); 

        assert_eq!(testpal.get_color('q').unwrap().to_string(), "#00ff00");
        assert_eq!(testpal.get_color('w').unwrap().to_string(), "#00ffffff");
        assert_eq!(testpal.get_color('e').unwrap().to_string(), "rgb(255,0,255)");
        assert_eq!(testpal.get_color('r').unwrap().to_string(), "rgba(255,0,0,136)");
        assert_eq!(testpal.get_color('t').unwrap().to_string(), "rgba(0,0,0,0)");
        assert_eq!(testpal.get_color('y').unwrap().to_string(), "#000000");
    }


    
}
