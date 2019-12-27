use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "U";


pub struct SchematicBus
{
    params : ItemParams
}


impl SchematicItem for SchematicBus
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicBus
{
    pub fn create(params: ItemParams) -> SchematicBus
    {
        SchematicBus { params }
    }
}
