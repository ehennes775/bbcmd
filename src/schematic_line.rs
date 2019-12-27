use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "L";


pub struct SchematicLine
{
    params : ItemParams
}


impl SchematicItem for SchematicLine
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicLine
{
    pub fn create(params: ItemParams) -> SchematicLine
    {
        SchematicLine { params }
    }
}
