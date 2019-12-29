use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;
use std::io::Write;


pub const CODE: &str = "N";


pub struct Net
{
    attributes : ItemAttributes,


    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Item for Net
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


impl Net
{
    pub fn create(params: ItemParams, reader : &mut impl ItemReader) -> Net
    {
        Net
        {
            attributes: ItemAttributes::read_from(reader).unwrap(),
            params
        }
    }
}
