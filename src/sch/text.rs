use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;
use std::io::Write;
use std::fmt::{Debug, Formatter, Error};


pub const CODE: &str = "T";
pub const NAME: &str = "Text";


pub struct Text
{
    lines : Vec<String>,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0,
    LINES = 9
}


impl Debug for Text
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        match self.lines.len()
        {
            1usize => write!(f, "{} {{ \"{}\" }}", NAME, &self.lines[0].trim_end()),
            l => write!(f, "{} {{ lines={} }}", NAME, l)
        }
    }
}


impl Item for Text
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


impl Text
{
    pub fn create(params: ItemParams, reader: &mut impl ItemReader) -> Result<Text,i32>
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

        Ok(Text { lines, params })
    }
}
