use serde_derive::Deserialize;
use std::fmt::Debug;
use serde::export::Formatter;
use serde::export::fmt::Error;


/// Contains manufacturer name and part number for a purchased part
#[derive(Deserialize)]
pub struct Part
{
    #[serde(rename="m")]
    pub manufacturer: String,


    #[serde(rename="p")]
    pub part_number: String
}


impl Debug for Part
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Part {{ m=\"{}\" p=\"{}\" }}", self.manufacturer, self.part_number)
    }
}


impl Part
{
    /// The name of the manufacturer
    pub fn manufacturer(&self) -> &String { &self.manufacturer }


    /// The manufacturer part number
    pub fn part_number(&self) -> &String { &self.part_number }
}