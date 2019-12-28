use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::{Write, Read, BufRead, BufReader, Error};


pub const CODE: &str = "T";


pub struct SchematicText
{
    lines : Vec<String>,


    params : ItemParams
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
    pub fn create<T: BufRead>(params: ItemParams, buffer : &mut String, reader: &mut T) -> Result<SchematicText,i32>
    {
        let count_param = String::from(&params[9]);
        let line_count = count_param.parse::<usize>().unwrap();
        let mut lines = vec![];

        for count in 0..line_count
        {
            lines.push(buffer.to_string());

            buffer.clear();
            let count2 = reader.read_line(buffer).unwrap();
        }

        Ok(SchematicText { lines, params })
    }
}
