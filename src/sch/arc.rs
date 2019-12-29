use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;
use std::fmt::{Debug, Formatter, Error};


pub const CODE: &str = "A";
pub const NAME: &str = "Arc";


pub struct Arc
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Arc
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { write!(f, "{}", NAME) }
}


impl Item for Arc
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl Arc
{
    pub fn create(params: ItemParams) -> Arc
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Arc { params }
    }
}
