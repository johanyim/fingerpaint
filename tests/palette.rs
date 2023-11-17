use csscolorparser::{self,Color};
use qolor::palette::Palette;
use qolor::color;

#[cfg(test)]
mod tests {
    use qolor::color::Color;

    use super::*;

    #[test]
    fn adds_and_gets_colors() {
        let mut testpal = Palette::new("testpal");
        let black = "000000";

        let _ = testpal.set('q', "green", color::Format::HEX, "rgb(0,255,0)"); 
        let _ = testpal.set('w', "cyan", color::Format::HEXA, "00ffff"); 
        let _ = testpal.set('e', "magenta", color::Format::RGB, "magenta"); 
        let _ = testpal.set('r', "red", color::Format::RGBA, "ff000088"); 
        let _ = testpal.set('t', "glass", color::Format::RGBA, "transparent"); 
        let _ = testpal.set('y', "black", color::Format::HEX, black); 

        assert_eq!(testpal.get_string('q'), "#00ff00");
        assert_eq!(testpal.get_string('w'), "#00ffffff");
        assert_eq!(testpal.get_string('e'), "rgb(255,0,255)");
        assert_eq!(testpal.get_string('r'), "rgba(255,0,0,136)");
        assert_eq!(testpal.get_string('t'), "rgba(0,0,0,0)");
        assert_eq!(testpal.get_string('y'), "#000000");
    }

    #[test]
    fn removes_colors() {
        let mut testpal = Palette::new("testpal");
        let _ = testpal.set('w', "grey", color::Format::HEXA, "rgba(128,128,128,0.01)"); 
        
        //testing that te color has been added
        assert_eq!(testpal.get_string('w'), "#80808003");

        let removed = testpal.remove('w');
        // the string that was just removed
        assert_eq!(removed.unwrap().to_string(), "#80808003");

        // color assigned to the key is now empty/black
        assert_eq!(testpal.get_name('w'), "");        
        assert_eq!(testpal.get_string('w'), "#000000");        
    }

    #[test]
    fn colors_are_not_borrowed() {
        let mut testpal = Palette::new("testpal");
        let _ = testpal.set('q', "blue", color::Format::HEX, "rgb(0,0,255)"); 
        assert_eq!(testpal.get_string('q'), "#0000ff");
        assert_eq!(testpal.get_string('q'), "#0000ff");
        assert_eq!(testpal.get_string('q'), "#0000ff");
    }

    #[test]
    fn adding_invalid_color() {
        let invalid_color = "#222aldskjagasdh".to_string();
        let mut testpal = Palette::new("testpal");
        testpal.set('q', "an invalid color", 
                            color::Format::RGBA, 
                            &invalid_color);
        
        // invalid color won't be added
        assert_ne!(testpal.get_name('q'), "an invalid color");
        assert_ne!(testpal.get_string('q'), invalid_color);

        // color will not be assigned
        assert!(testpal.colors.get(&'q').is_none());
        assert_eq!(testpal.get_name('q'), "");
        assert_eq!(testpal.get_string('q'), "#000000");
    }

    #[test]
    fn not_overwritten_by_invalid_color() {
        let valid_color = "#FFFF00".to_string();
        let invalid_color = "#FFalFkjagasdh".to_string();
        let mut testpal = Palette::new("testpal");

        testpal.set('q', "yellow", 
                            color::Format::HEXA, 
                            &valid_color);
        
        assert_eq!(testpal.get_name('q'), "yellow");
        assert_eq!(testpal.get_string('q'), "#ffff00ff");

        testpal.set('q', "invalid", 
                            color::Format::RGB, 
                            &invalid_color);

        assert_ne!(testpal.get_name('q'), "invalid");
        assert_ne!(testpal.get_string('q'), "rgb(FFFF00)");

        assert_eq!(testpal.get_name('q'), "yellow");
        assert_eq!(testpal.get_string('q'), "#ffff00ff");
    }

    #[test]
    fn changes_output_format() {
        let mut testpal = Palette::new("testpal");
        testpal.set('q', "transparent yellow", 
                            color::Format::HEX, 
                            "#ffff0080");
        // color no longer shows alpha channel, though it is still remembered
        assert_eq!(testpal.get_string('q'), "#ffff00");
        testpal.change_format('q', color::Format::RGBA);
        // alpha channel is not forgoetten after switching formats
        assert_eq!(testpal.get_string('q'), "rgba(255,255,0,128)"); 
    }

    #[test]
    fn cmyk_format() {
        let mut testpal = Palette::new("testpal");
        testpal.set('q', 
            "navy",
            color::Format::CMYK,
            "#2D4192"
            ); 
        assert_eq!(testpal.get_string('q'), "cmyk(69%, 55%, 0%, 43%)")
    }
    
    #[test]
    fn hsl_format() {
        let mut testpal = Palette::new("testpal");

        testpal.set('q', 
            "brown",
            color::Format::HSL,
            "rgb(100,21,12)"
            ); 
        assert_eq!(testpal.get_string('q'), "hsl(6deg 79% 22%)");

        testpal.set('w', 
            "transparent navy",
            color::Format::HSL,
            "#03044f45"
            ); 
        assert_eq!(testpal.get_string('w'), "hsl(239deg 93% 16% / 0.27)");

        testpal.set('e', 
            "opaque green",
            color::Format::HSL,
            "#00ff00ff"
            ); 
        assert_eq!(testpal.get_string('e'), "hsl(120deg 100% 50%)");

        testpal.set('r', 
            "half transparent blue",
            color::Format::HSL,
            "#0000ff80"
            ); 
        assert_eq!(testpal.get_string('r'), "hsl(240deg 100% 50% / 0.50)");

        testpal.set('t', 
            "opaque white",
            color::Format::HSL,
            "rgb(255,255,255)"
            ); 
        assert_eq!(testpal.get_string('t'), "hsl(0deg 0% 100%)");

        testpal.set('y', 
            "opaque grey",
            color::Format::HSL,
            "rgb(128,128,128)"
            ); 
        assert_eq!(testpal.get_string('y'), "hsl(0deg 0% 50%)");

        testpal.set('u', 
            "opaque black",
            color::Format::HSL,
            "rgb(0,0,0)"
            ); 
        assert_eq!(testpal.get_string('u'), "hsl(0deg 0% 0%)");
    }

    #[test]
    fn deletes_alpha() {
        let mut testpal = Palette::new("testpal");

        testpal.set('q', 
            "blue",
            color::Format::HEXA,
            "rgba(0,0,255,0.3)"
            ); 
        assert_eq!(testpal.get_string('q'), "#0000ff4d");
        testpal.delete_alpha('q');
        assert_eq!(testpal.get_string('q'), "#0000ffff");
        
    }
    

}

