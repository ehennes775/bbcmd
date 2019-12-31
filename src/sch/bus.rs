use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;
use std::io::Write;
use std::fmt::{Formatter, Debug, Error};


pub const CODE: &str = "U";
pub const NAME: &str = "Bus";


pub struct Bus
{
    attributes : ItemAttributes,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Bus
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        match self.attributes.items.len()
        {
            1usize => write!(f, "{} {{ {:?} }}", NAME, self.attributes.items[0]),
            c => write!(f, "{} {{ attributes={} }}", NAME, c)
        }

    }
}


impl Item for Bus
{
    fn attributes(&self) -> Option<&ItemAttributes> { Some(&self.attributes) }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { Some(&mut self.attributes) }


    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>) -> std::io::Result<()>
    {
        self.params.write_to(writer)?;
        self.attributes.write_to(writer)?;

        Ok(())
    }
}


impl Bus
{
    pub fn create(params: ItemParams, reader : &mut impl ItemReader) -> Bus
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Bus
        {
            attributes: ItemAttributes::read_from(reader).unwrap(),
            params
        }
    }
}
