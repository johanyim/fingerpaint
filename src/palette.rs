extern crate serde;
extern crate serde_yaml;

use filenamify::filenamify;

use serde::{Serialize,Deserialize};

use std::fs;

use std::collections::HashMap;
use std::io::{Write, BufReader};
use crate::color::Color;


#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: HashMap<char, Color>,
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


    pub fn get_color(self, key: char) -> String {
        return self.colors.get(&key).unwrap().to_string();
    
    }
}





