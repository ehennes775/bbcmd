use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub struct SchematicBox
{
    params : ItemParams
}


impl SchematicItem for SchematicBox
{
    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicBox
{
    pub fn create(params: ItemParams) -> SchematicBox
    {
        SchematicBox { params }
    }
}
