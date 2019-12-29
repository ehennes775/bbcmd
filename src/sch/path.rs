use crate::sch::item::Item;
use crate::sch::item_params::ItemParams;
use crate::sch::reader::ItemReader;
use std::io::Write;
use std::fmt::{Formatter, Debug, Error};


pub const CODE: &str = "H";
pub const NAME: &str = "Path";


pub struct Path
{
    lines : Vec<String>,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0,
    LINES = 13
}


impl Debug for Path
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "{} {{ lines={} }}", NAME, &self.lines.len())
    }
}


impl Item for Path
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);

        for line in &self.lines
        {
            writer.write(line.as_bytes());
        }
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

        Ok(Path { lines, params })
    }
}
