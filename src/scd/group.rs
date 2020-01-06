use serde_derive::Deserialize;
use crate::scd::part::Part;
use std::fmt::{Debug, Formatter, Error};


#[derive(Deserialize)]
pub struct Group
{
    pub parts: Vec<Part>,
    pub value: String
}


impl Debug for Group
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Group {{ count={} value=\"{}\" }}", &self.parts.len(), &self.value)
    }
}


impl Group
{
    /// The parts within the group
    pub fn parts(&self) -> std::vec::IntoIter<&Part> { self.parts().into_iter() }


    /// The value shared by all parts within this group
    pub fn value(&self) -> &str { &self.value }
}