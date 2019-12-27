use crate::item_params::ItemParams;
use crate::schematic_item::SchematicItem;
use std::io::{Write, Read, BufRead, BufReader, Error};


pub struct SchematicText
{
    lines : Vec<String>,


    params : ItemParams
}


impl SchematicItem for SchematicText
{
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
    pub fn create<T: BufRead>(params: ItemParams, reader: &mut T) -> Result<SchematicText,&str>
    {
        let count_param = String::from(&params[9]);

        let line_count = count_param.parse::<usize>().unwrap();

        let mut buffer = String::new();
        let mut lines:Vec<String> = Vec::new();

        for count in 0..line_count
        {
            let mut line = reader.read_line(&mut buffer);

            match line
            {
                Err(_r) => {},
                Ok(_t) => lines.push(String::from(&buffer))
            }

            buffer.clear()
        }

        Ok(SchematicText { lines, params })
    }
}
