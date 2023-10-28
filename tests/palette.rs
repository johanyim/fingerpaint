use csscolorparser::{self,Color};
use qolor::palette::Palette;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_colors() {
        let mut testpal = Palette::new("testpal");
        let black = csscolorparser::parse("#111111").unwrap();

        testpal.add('q', "black");

        assert_eq!(testpal.get_color('q'), "black");

    }
}
