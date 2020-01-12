use crate::library::library::Library;
use crate::scd::drawing::Drawing;
use crate::sch::page::Page;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};
use serde_derive::Deserialize;
use std::env;


#[derive(Deserialize)]
pub struct Project
{
    #[serde(rename="drawingLib")]
    drawings : Library,


    #[serde(rename="symbolLib")]
    symbols : Library
}


impl Project
{
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Project,Box<std::error::Error>>
    {
        let file = File::open(path).unwrap();

        let mut reader = BufReader::new(file);

        let mut contents = String::new();
        let x = reader.read_to_string(&mut contents).unwrap();

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
    use std::env;
    use std::path::PathBuf;
    use super::*;


    #[test]
    fn test()
    {
    }
}