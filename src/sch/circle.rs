use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;
use std::fmt::{Debug, Formatter, Error};


pub const CODE: &str = "V";
pub const NAME: &str = "Circle";


pub struct Circle
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Circle
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { write!(f, "{}", NAME) }
}


impl Item for Circle
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl Circle
{
    pub fn create(params: ItemParams) -> Circle
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Circle { params }
    }
}
