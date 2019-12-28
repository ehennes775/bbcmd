use crate::item_attributes::ItemAttributes;
use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


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
    pub fn create(params: ItemParams) -> SchematicPin { SchematicPin { attributes: ItemAttributes::new(), params } }
}
