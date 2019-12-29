use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;


pub const CODE: &str = "A";


pub struct SchematicArc
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl SchematicItem for SchematicArc
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicArc
{
    pub fn create(params: ItemParams) -> SchematicArc
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        SchematicArc { params }
    }
}
