use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "A";


pub struct SchematicPin
{
    params : ItemParams
}


impl SchematicItem for SchematicPin
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>) { self.params.write_to(writer); }
}


impl SchematicPin
{
    pub fn create(params: ItemParams) -> SchematicPin { SchematicPin { params } }
}
