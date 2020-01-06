use crate::scd::group::Series;
use crate::scd::part::Part;
use serde_derive::Deserialize;
use std::fmt::Debug;
use serde::export::Formatter;
use serde::export::fmt::Error;
use crate::library::loadable::Loadable;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;


#[derive(Deserialize)]
pub struct Drawing
{
    #[serde(rename="description")]
    pub description: String,


    #[serde(rename="groups")]
    pub groups: Vec<Series>
}


impl Debug for Drawing
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Part {{ description=\"{}\" }}", self.description)
    }
}


impl Drawing
{
    pub fn description(&self) -> &String { &self.description }


    pub fn find_parts(&self, value: &str) -> std::vec::IntoIter<&Part>
    {
        let parts = self.groups.iter()
            .filter(|s| s.value.eq(value))
            .flat_map(|s| s.parts.iter())
            .collect::<Vec<_>>();

        parts.into_iter()
    }
}


impl Loadable for Drawing
{
    fn load(path: &PathBuf) -> Result<Box<Self>, Box<std::error::Error>>
    {
        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);

        let u: Drawing = serde_json::from_reader(reader).unwrap();

        Ok(Box::new(u))
    }
}