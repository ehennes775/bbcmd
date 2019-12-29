use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "L";


pub struct SchematicLine
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
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
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        SchematicLine { params }
    }
}
