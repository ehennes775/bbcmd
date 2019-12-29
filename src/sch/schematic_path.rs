use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::io::Write;
use crate::sch::schematic_reader::ItemReader;


pub const CODE: &str = "H";


pub struct SchematicPath
{
    params : ItemParams
}


enum ParamIndex
{
    CODE = 0,
    LINES = 13
}


impl SchematicItem for SchematicPath
{
    fn params(&self) -> &ItemParams { &self.params }


    fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        self.params.write_to(writer);
    }
}


impl SchematicPath
{
    pub fn create(params: ItemParams, reader: &mut impl ItemReader) -> Result<SchematicPath,i32>
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

        Ok(SchematicPath { params })
    }
}
