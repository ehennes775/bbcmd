use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "B";


pub struct SchematicBox
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl SchematicItem for SchematicBox
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicBox
{
    pub fn create(params: ItemParams) -> SchematicBox
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        SchematicBox { params }
    }
}
