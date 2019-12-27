use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub struct SchematicArc
{
    params : ItemParams
}


impl SchematicItem for SchematicArc
{
    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicArc
{
    pub fn create(params: ItemParams) -> SchematicArc
    {
        SchematicArc { params }
    }
}
