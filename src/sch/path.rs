use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::io::Write;
use crate::sch::reader::ItemReader;


pub const CODE: &str = "H";


pub struct Path
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0,
    LINES = 13
}


impl Item for Path
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl Path
{
    pub fn create(params: ItemParams, reader: &mut impl ItemReader) -> Result<Path,i32>
    {
        assert_eq!(&params[ParamIndex::CODE as usize], CODE);

        let param = String::from(&params[ParamIndex::LINES as usize]);

        let count = match param.parse::<usize>()
        {
            Err(_e) => return Err(42),
            Ok(t) => t
        };

        let lines = match reader.read_lines(count)
        {
            Err(_e) => return Err(42),
            Ok(t) => t
        };

        Ok(Path { params })
    }
}
