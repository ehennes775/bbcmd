use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub struct SchematicComplex
{
    params : ItemParams
}


impl SchematicItem for SchematicComplex
{
    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicComplex
{
    pub fn create(params: ItemParams) -> SchematicComplex
    {
        SchematicComplex { params }
    }
}
