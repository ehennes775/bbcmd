use crate::item_attributes::ItemAttributes;
use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::{Write, BufRead};


pub const CODE: &str = "U";


pub struct SchematicBus
{
    attributes : ItemAttributes,


    params : ItemParams
}


impl SchematicItem for SchematicBus
{
    fn attributes(&self) -> Option<&ItemAttributes> { Some(&self.attributes) }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { Some(&mut self.attributes) }


    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicBus
{
    pub fn create<T: BufRead>(params: ItemParams, buffer: &mut String, reader: &mut T) -> SchematicBus
    {
        SchematicBus
        {
            attributes: ItemAttributes::read_from(buffer, reader).unwrap(),
            params
        }
    }
}
