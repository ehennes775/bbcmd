use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use crate::sch::schematic_reader::ItemReader;
use std::io::Write;


pub const CODE: &str = "T";


pub struct SchematicText
{
    lines : Vec<String>,

    params : ItemParams
}


enum ParamIndex
{
    CODE = 0,
    LINES = 9
}


impl SchematicItem for SchematicText
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


impl SchematicText
{
    pub fn create(params: ItemParams, reader: &mut impl ItemReader) -> Result<SchematicText,i32>
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

        Ok(SchematicText { lines, params })
    }
}
