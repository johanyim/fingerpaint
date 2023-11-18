use std::path::Path;
use std::fs;


pub struct Tabline {
    titles: Vec<String>,
    paths: Vec<String>,
    working_directory: String,
}

impl Tabline {

    pub fn new(dir: &str) -> Self {
        Tabline {
            titles: vec![],
            paths: vec![],
            working_directory: dir.to_string(),
        }
    }

    pub fn get_titles(&mut self) -> std::io::Result<()>{
        let palette_paths = fs::read_dir(&self.working_directory)?;
        for entry in palette_paths {
            let dir = entry?;
            println!("{:?}", dir.path());

        }
        return Ok(())
    }
}
