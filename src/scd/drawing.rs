use crate::scd::group::Group;
use crate::scd::part::Part;
use serde_derive::Deserialize;
use std::fmt::Debug;
use serde::export::Formatter;
use serde::export::fmt::Error;
use crate::library::loadable::Loadable;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;


#[derive(Deserialize)]
pub struct Drawing
{
    #[serde(rename="description")]
    pub description: String,


    #[serde(rename="groups")]
    pub groups: Vec<Group>
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
    /// A description of this drawing without macro expansion
    pub fn description(&self) -> &String { &self.description }


    /// Find all parts with a specific value
    pub fn find_parts(&self, value: &str) -> std::vec::IntoIter<&Part>
    {
        self.groups.iter()
            .filter(|s| s.value().eq(value))
            .flat_map(|s| s.parts())
            .collect::<Vec<_>>()
            .into_iter()
    }
}


impl Loadable for Drawing
{
    fn load<T: AsRef<Path>>(path: T) -> Result<Box<Self>, Box<dyn std::error::Error>>
    {
        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);

        let u: Drawing = serde_json::from_reader(reader).unwrap();

        Ok(Box::new(u))
    }
}