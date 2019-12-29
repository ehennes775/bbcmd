use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;
use std::io::Write;
use std::fmt::{Formatter, Debug, Error};


pub const CODE: &str = "C";
pub const NAME: &str = "Complex";


pub struct Complex
{
    attributes : ItemAttributes,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Complex
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


impl Item for Complex
{
    fn attributes(&self) -> Option<&ItemAttributes> { Some(&self.attributes) }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { Some(&mut self.attributes) }


    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
        self.attributes.write_to(writer);
    }
}


impl Complex
{
    pub fn create<T: ItemReader>(params: ItemParams, reader : &mut T) -> Complex
    {
        Complex
        {
            attributes: ItemAttributes::read_from(reader).unwrap(),
            params
        }
    }
}
