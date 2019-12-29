use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use crate::sch::schematic_reader::ItemReader;
use std::io::Write;


pub const CODE: &str = "C";


pub struct SchematicComplex
{
    attributes : ItemAttributes,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl SchematicItem for SchematicComplex
{
    fn attributes(&self) -> Option<&ItemAttributes> { Some(&self.attributes) }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { Some(&mut self.attributes) }


    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
        self.attributes.write_to(writer);
    }
}


impl SchematicComplex
{
    pub fn create<T: ItemReader>(params: ItemParams, reader : &mut T) -> SchematicComplex
    {
        SchematicComplex
        {
            attributes: ItemAttributes::read_from(reader).unwrap(),
            params
        }
    }
}
