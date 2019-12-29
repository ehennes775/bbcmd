use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "V";


pub struct SchematicCircle
{
    params : ItemParams
}


impl SchematicItem for SchematicCircle
{
    fn params(&self) -> &ItemParams { &self.params }


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
