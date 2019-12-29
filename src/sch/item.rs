use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use std::io::Write;


pub trait Item
{
    fn attributes(&self) -> Option<&ItemAttributes> { None }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { None }


    fn params(&self) -> &ItemParams;


    fn write_to(&self, writer: &mut Box<dyn Write>);
}
