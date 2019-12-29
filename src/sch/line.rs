use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;


pub const CODE: &str = "L";


pub struct Line
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Item for Line
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
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
