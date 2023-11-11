extern crate serde;
extern crate serde_yaml;

use csscolorparser::ParseColorError;
use filenamify::filenamify;
use serde::{Serialize,Deserialize};
use std::{fs, path};
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::path::PathBuf;
use crate::color::{self, Color, Format};
use crate::config::Config;

use ratatui::style::{Style, Color as RatatuiColor};

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: HashMap<char, color::Color>,
}

impl Palette {
    pub fn new(name: &str) -> Self {
        let colors: HashMap<char, Color> = HashMap::new();
        return Palette{name: name.to_string(), colors};
    }
    
    pub fn save(&self, config: &Config) -> std::io::Result<()> {
        let mut path = PathBuf::from(&config.palettes);
        path.push(filenamify(self.name.clone()));
        path.set_extension("yaml");

        let mut output = fs::File::create(path.as_path())?;
        let yaml = serde_yaml::to_string(&self).unwrap();
        
        // write!(output, "{}", yaml) 
        output.write_all(&yaml.into_bytes())?;
        Ok(())
    }

    pub fn load(config: &Config) -> Result<Palette, std::io::Error> {

        let mut path = PathBuf::from(&config.palettes);
        path.push(filenamify(&config.selected));
        path.set_extension("yaml");

        let file = fs::File::open(path.as_path()).expect("Could not find file");
        let reader = BufReader::new(file);

        let loaded: Palette = serde_yaml::from_reader::<_, Palette>(reader)
            .expect("Could not read file");

        return Ok(loaded);
    } 

    pub fn set(&mut self, key: char, name: &str, format: color::Format, content: &str ) -> Result<(), ParseColorError>{
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

    pub fn get_rgba(&self, key: char) -> [u8;4] {
        let value = self.colors.get(&key);
        match value {
            Some(color) => color.rgba_color,
            None => [0,0,0,0], 
        }    
    }

    pub fn remove(&mut self, key: char) -> Option<Color>{
        self.colors.remove(&key)
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



