use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;


pub const CODE: &str = "A";


pub struct Arc
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0
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
