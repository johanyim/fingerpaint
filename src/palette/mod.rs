pub mod reader;

use csscolorparser::ParseColorError;
use filenamify::filenamify;
use serde::{Serialize,Deserialize};
use std::fs::{self, DirEntry};
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::path::{PathBuf, Path};
use crate::color::{self, Color, Format};
use crate::config::Config;
use crate::error::PaletteError::{
    self, CouldNotCreate, CouldNotWrite, 
    PalettesPathNotFound, CouldNotFind, CouldNotParse};

use ratatui::style::{Style, Color as RatatuiColor};

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: HashMap<char, color::Color>,
}

impl Default for Palette {
    fn default() -> Self {
        Palette { 
            name: "Default".to_string(), 
            colors: HashMap::from([]) }
    }
}

impl Palette { 

    pub fn new(name: &str) -> Self {
        let colors: HashMap<char, Color> = HashMap::new();
        return Palette{name: name.to_string(), colors};
    }
    
    pub fn save(&self, config: &Config) -> Result<(), PaletteError> {
        let mut path = match &config.palettes {
            Some(plts) => PathBuf::from(plts),
            None => {
                return Err(PalettesPathNotFound)
            },
        };
        path.push(filenamify(self.name.clone()));
        path.set_extension("yaml");

        //variable only used for debugging
        let pathstring = path.as_path()
            .to_str()
            .unwrap_or("undefined")
            .to_string();

        let mut output = match fs::File::create(path.as_path()) {
            Err(e) => return Err(CouldNotCreate(e, pathstring)),
            Ok(out) => out,
        };
        let yaml = serde_yaml::to_string(&self).unwrap();
        
        // write!(output, "{}", yaml) 
        if let Err(e) = output.write_all(&yaml.into_bytes()){
            return Err(CouldNotWrite(e, pathstring))
        }
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Palette, PaletteError> {

        let pathstring = path.to_str()
            .unwrap_or("undefined")
            .to_string();

        let file = match fs::File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(CouldNotFind(e, pathstring)),
        };

        let reader = BufReader::new(file);

        match serde_yaml::from_reader::<_, Palette>(reader) {
            Ok(file) => return Ok(file),
            Err(e) => return Err(CouldNotParse(e, pathstring)),
        };


    }
    pub fn load_selected(config: &Config) -> Result<Palette, PaletteError> {
        // let mut path = PathBuf::from(&config.palettes);
        let mut path = match &config.palettes {
            Some(plts) => PathBuf::from(plts),
            None => return Ok(Palette::default()),
        };
        match &config.selected {
            Some(plt) => path.push(filenamify(plt)),
            None => return Ok(Palette::default()),
        };
        path.set_extension("yaml");
        return Palette::load(path.as_path());

    } 
    pub fn load_all(config: &Config) -> Result<Vec<Palette>, PaletteError>{
        // let palette_paths = fs::read_dir(&config.palettes)?;
        
        // This is "~/config/fingerpaint/palletes/"
        let path = match &config.palettes {
            Some(plts) => plts,
            None => return Ok(vec![]),
        };
        
        let palettes: Vec<Palette> = fs::read_dir(path).unwrap()
            .into_iter()
            .filter_map(|f| f.ok())
            .map(|direntry| direntry.path())
            .map(|path| Palette::load(&path)).filter_map(|f| f.ok())
            .collect();

        return Ok(palettes);
    }


    pub fn set(&mut self, key: char, name: &str, 
               format: color::Format, content: &str 
               ) -> Result<(), ParseColorError>{
        let content = Color::new(name, format, content);
        match content {
            Ok(color) => self.colors.insert(key, color),
            Err(e) => return Err(e),
        };
        return Ok(());
    }
    
    pub fn get_name(&self, key: char) -> String {
        let value = self.colors.get(&key);
        match value {
            Some(color) => color.name.to_string(),
            None => "".to_string(), 
        }    
    }
    
    //if returned None, prompt user for new color
    pub fn get_string(&self, key: char) -> String {
        let value = self.colors.get(&key);
        match value {
            Some(color) => color.to_string(),
            None => "#000000".to_string(), 
        }    
    }
    pub fn remove(&mut self, key: char) -> Option<Color>{
        self.colors.remove(&key)
    }

    pub fn get_rgba(&self, key: char) -> [u8;4] {
        let value = self.colors.get(&key);
        match value {
            Some(color) => color.rgba_color,
            None => [0,0,0,0], 
        }    
    }


    pub fn change_format(&mut self, key: char, format: Format) {
        if let Some(color) = self.colors.get_mut(&key) {
             color.output_format = format 
        }
    }

    pub fn delete_alpha(&mut self, key: char) {
        if let Some(color) = self.colors.get_mut(&key) {
            color.rgba_color[3] = 255;
        }
    }
    
    pub fn get_displayable(&self, key: char) -> Style {
        let value = self.colors.get(&key);
        match value {
            None => Style::default(), 
            Some(color) => {
                // whether the text should be white or black
                // depends on the background
                if color.rgba_color[0] as u16 
                    + color.rgba_color[1] as u16
                    + color.rgba_color[2] as u16 > (255+255+255)/2 {
                    Style::default()
                        .fg(RatatuiColor::Black)
                        .bg(RatatuiColor::Rgb(
                                color.rgba_color[0],
                                color.rgba_color[1],
                                color.rgba_color[2]
                                ))
                }else{
                    Style::default()
                        .fg(RatatuiColor::White) 
                        .bg(RatatuiColor::Rgb(
                                color.rgba_color[0],
                                color.rgba_color[1],
                                color.rgba_color[2]
                                ))

                }
            },
        }    
    }
}



