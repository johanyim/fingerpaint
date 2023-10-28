extern crate serde;
extern crate serde_yaml;

use filenamify::filenamify;

use serde::{Serialize,Deserialize};

use std::fs;

use std::collections::HashMap;
use std::io::{Write, BufReader, Error};
use csscolorparser;

use crate::color::{self, Color};


#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: HashMap<char, color::Color>,
}


impl Palette {
    //makes a new file?
    pub fn new(name: &str) -> Self {
        let colors: HashMap<char, Color> = HashMap::new();
        return Palette{name: name.to_string(), colors};
    }
    
    //write out to file
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


    pub fn add(&mut self, key: char, color: &str ) -> Result<(), Error>{
        //ensure all colors are uniformly stored as rgb

        let col = Color::new("unnamed", color);

        self.colors.insert(key, col);
        Ok(())
    }


    pub fn get_color(self, key: char) -> Option<String>{
        let color = self.colors.get(&key)
            .expect("Expected a color");
        return Some(color.to_string());
    }
}



