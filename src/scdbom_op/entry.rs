use crate::scdbom_op::key::Key;
use crate::sch::complex::Complex;
use crate::refdes_op::refdes::Refdes;
use crate::scd::part::Part;
use crate::scd::drawing::Drawing;
use regex::Regex;
use std::fmt::{Debug, Formatter, Error};


/// Contains a line entry in an engineering bill of materials (BOM)
pub struct Entry<'a>
{
    /// All the schematic symbols representing components on this line entry
    components: Vec<&'a Complex>,


    /// The specification control drawing for all components in this line entry
    drawing: &'a Drawing,


    /// Does this need to still be here?
    key: &'a Key
}


impl<'a> Debug for Entry<'a>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Entry {{ key={:?} }}", self.key)
    }
}


impl<'a> Entry<'a>
{
    /// Creates a new engineering bill of material line item
    ///
    /// # Arguments
    ///
    /// * `key` -
    /// * `drawing` -
    /// * `c` -
    pub fn new(key: &'a Key, drawing: &'a Drawing, c: Vec<&'a Complex>) -> Entry<'a>
    {
        Entry { key, drawing, components: c }
    }


    /// The description of the part, with macros expanded
    pub fn description(&self) -> String
    {
        lazy_static!
        {
            static ref REGEX: Regex = Regex::new(r"\$\(value\)").unwrap();
        }

        match &self.key.value
        {
            None => String::from(self.drawing.description()),
            Some(t) => REGEX.replace_all(self.drawing.description(), t.as_str()).to_string()
        }
    }


    /// Reference designators for all components on this line item
    pub fn refdes(&self) -> std::vec::IntoIter<Refdes>
    {
        let mut refdes_attributes = self.components.iter()
            .flat_map(|complex| complex.attributes.items.iter())
            .filter(|text| text.attribute_name().is_some())
            .filter(|text| text.attribute_name().unwrap().eq(&String::from(r"refdes")))
            .map(|text| text.attribute_value())
            .flat_map(|x| x)
            .map(|r| r.parse::<Refdes>())
            .flat_map(|x| x)
            .collect::<Vec<_>>();

        refdes_attributes.sort();

        refdes_attributes.into_iter()
    }


    /// Corresponding purchased parts for all components on this line item
    pub fn parts(&self) -> std::vec::IntoIter<&Part>
    {
        let mut parts = self.drawing.find_parts(self.value())
            .collect::<Vec<_>>();

        parts.sort_by_key(|p| p.manufacturer());

        parts.into_iter()
    }


    /// Returns the filename of the specification control drawing (SCD)
    pub fn scd(&self) -> &str
    {
        &self.key.scd()
    }


    /// Returns the value of the components in this entry, or an empty string if none
    pub fn value(&self) -> &str
    {
        match &self.key.value
        {
            None => "",
            Some(t) => t.as_str()
        }
    }
}