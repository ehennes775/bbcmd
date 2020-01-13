use crate::library::library::Library;
use crate::scd::drawing::Drawing;
use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, Read};
use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Project
{
    #[serde(rename="drawingLib")]
    drawings : Library,


    #[serde(rename="symbolLib")]
    _symbols : Library
}


impl Project
{
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Project,Box<dyn std::error::Error>>
    {
        let file = File::open(path).unwrap();

        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        let _x = reader.read_to_string(&mut contents).unwrap();

        let project: Project = toml::from_str(&contents).unwrap();

        Ok(project)
    }


    pub fn load_drawing(&self, name: &str) -> Result<Drawing, Box<dyn std::error::Error>>
    {
        self.drawings.load_item(name)
    }
}


#[cfg(test)]
mod test
{
    #[test]
    fn test()
    {
    }
}