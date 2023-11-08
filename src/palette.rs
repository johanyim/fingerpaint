extern crate serde;
extern crate serde_yaml;

use filenamify::filenamify;
use serde::{Serialize,Deserialize};
use std::fs;
use std::collections::HashMap;
use std::io::{Write, BufReader};
use crate::color::{self, Color, Format};

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
    
    pub fn save(self) -> std::io::Result<()> {
        let safe_filename = filenamify(self.name.clone()) + &".yaml";
        let mut output = fs::File::create(safe_filename)?;
        let yaml = serde_yaml::to_string(&self).unwrap();
        
        // write!(output, "{}", yaml) 
        output.write_all(&yaml.into_bytes())?;
        Ok(())
    }

    pub fn load(palletename: &str) -> Result<Palette, std::io::Error> {
        let filename = filenamify(palletename) + &".yaml";
        
        let file = fs::File::open(filename).expect("Could not find file");
        let reader = BufReader::new(file);

        let loaded: Palette = serde_yaml::from_reader::<_, Palette>(reader)
            .expect("Could not read file");

        return Ok(loaded);
    } 

    pub fn add(&mut self, key: char, name: &str, format: color::Format, color: &str ){
        let col = Color::new(name, format, color);
        match col {
            Ok(color) => {self.colors.insert(key, color); return},
            Err(e) => {eprintln!("{e}: '{color}' was not a valid color")},
        };
    }
    
    pub fn get_name(&self, key: char) -> String {
        let value = self.colors.get(&key);
        match value {
            Some(color) => color.name.to_string(),
            None => "Unassigned".to_string(), 
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
                        .fg(RatatuiColor::Black) //TODO: Should be an averaged color
                        .bg(RatatuiColor::Rgb(
                                color.rgba_color[0],
                                color.rgba_color[1],
                                color.rgba_color[2]
                                ))
                }else{
                    Style::default()
                        .fg(RatatuiColor::White) //TODO: Should be an averaged color
                        .bg(RatatuiColor::Rgb(
                                color.rgba_color[0],
                                color.rgba_color[1],
                                color.rgba_color[2]
                                ))

                }
            },
        }    
    }
    // pub fn displayable(&self) -> Style {
    //     return Style::default()
    //         .fg(RatatuiColor::Black) //TODO: Should be an averaged color
    //         .bg(RatatuiColor::Rgb(self.rgba_color[0],self.rgba_color[1],self.rgba_color[2]));
    // }
}



