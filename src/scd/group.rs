use serde_derive::Deserialize;
use crate::scd::part::Part;


#[derive(Deserialize)]
pub struct Series
{
    pub value: String,


    pub parts: Vec<Part>
}
