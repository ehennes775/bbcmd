use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub struct SchematicCircle
{
    params : ItemParams
}


impl SchematicItem for SchematicCircle
{
    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicCircle
{
    pub fn create(params: ItemParams) -> SchematicCircle
    {
        SchematicCircle { params }
    }
}
