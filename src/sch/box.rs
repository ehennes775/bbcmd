use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;


pub const CODE: &str = "B";


pub struct Box
{
    params: ItemParams
}


enum ParamIndex
{
    CODE = 0
}


impl Item for Box
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut std::boxed::Box<dyn Write>)
    {
        self.params.write_to(writer);
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

