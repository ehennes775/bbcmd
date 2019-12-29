use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::{Write};
use crate::sch::reader::ItemReader;


pub const CODE: &str = "P";


pub struct Pin
{
    attributes : ItemAttributes,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Item for Pin
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


impl Pin
{
    pub fn create(params: ItemParams, reader : &mut impl ItemReader) -> Pin
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Pin
        {
            attributes: ItemAttributes::read_from(reader).unwrap(),
            params
        }
    }
}
