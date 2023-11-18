use super::Palette;
use std::fs;


pub struct PaletteReader {
    path: String,
    palettes: Vec<Palette>,
}


impl PaletteReader {
    pub fn new(path: String) -> Self {
        PaletteReader {
            path,
            palettes: vec![],
        }
    }



    // pub fn get_palettes(&mut self) -> std::io::Result<()>{
    //     let palette_paths = fs::read_dir(&self.path)?;
    //     self.palettes = palette_paths
    //         .into_iter().filter_map(|f| f.ok())
    //         .map(|f| toml)
    //         .collect();
    //
    //     return Ok(())
    // }




}
