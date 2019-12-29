use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "H";


pub struct SchematicPath
{
    params : ItemParams
}


impl SchematicItem for SchematicPath
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicPath
{
    pub fn create(params: ItemParams) -> SchematicPath
    {
        SchematicPath { params }
    }
}
