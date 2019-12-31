use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;
use std::fmt::{Formatter, Debug, Error};


pub const CODE: &str = "L";
pub const NAME: &str = "Line";


pub struct Line
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Debug for Line
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { write!(f, "{}", NAME) }
}


impl Item for Line
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>) -> std::io::Result<()>
    {
        self.params.write_to(writer)?;

        Ok(())
    }
}


impl Line
{
    pub fn create(params: ItemParams) -> Line
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        Line { params }
    }
}
