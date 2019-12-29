use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::{Write, BufRead};


pub const CODE: &str = "P";


pub struct SchematicPin
{
    attributes : ItemAttributes,


    params : ItemParams
}


impl SchematicItem for SchematicPin
{
    fn attributes(&self) -> Option<&ItemAttributes> { Some(&self.attributes) }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { Some(&mut self.attributes) }


    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>) { self.params.write_to(writer); }
}


impl SchematicPin
{
    pub fn create<T: BufRead>(params: ItemParams, buffer: &mut String, reader: &mut T) -> SchematicPin
    {
        SchematicPin
        {
            attributes: ItemAttributes::read_from(buffer, reader).unwrap(),
            params
        }
    }
}
