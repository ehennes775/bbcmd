use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;
use std::fmt::{Debug, Formatter, Error};


pub const CODE: &str = "B";
pub const NAME: &str = "Box";


pub struct Box
{
    params: ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Box
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { write!(f, "{}", NAME) }
}


impl Item for Box
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut std::boxed::Box<dyn Write>) -> std::io::Result<()>
    {
        self.params.write_to(writer)?;

        Ok(())
    }
}


impl Box
{
    pub fn create(params: ItemParams) -> Box
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Box { params }
    }
}

