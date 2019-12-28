use regex::Regex;
use std::io::{Write, BufRead};
use crate::schematic_text::SchematicText;
use crate::schematic_item::SchematicItem;
use crate::item_params::ItemParams;
use std::str::FromStr;
use crate::schematic_text;


pub struct ItemAttributes
{
    ending_line : String,


    items : Vec<SchematicText>,


    starting_line : String
}


impl ItemAttributes
{
    fn is_ending_line(line: &str) -> bool
    {
        lazy_static!
        {
            static ref REGEX: Regex = Regex::new(r"^\s*\}\s*$").unwrap();
        }

        REGEX.is_match(line)
    }


    fn is_starting_line(line: &str) -> bool
    {
        lazy_static!
        {
            static ref REGEX: Regex = Regex::new(r"^\s*\{\s*$").unwrap();
        }

        REGEX.is_match(line)
    }


    pub fn read_from<T: BufRead>(&mut self, buffer: &mut String, reader: &mut T)
    {
        if Self::is_starting_line(buffer)
        {
            self.starting_line = buffer.to_string();

            buffer.clear();
            let mut count = reader.read_line(buffer);

            while !Self::is_ending_line(buffer)
            {
                let params = ItemParams::from_str(buffer).unwrap();

                if params.code() == schematic_text::CODE
                {
                    let attribute = SchematicText::create(params, reader).unwrap();

                    self.items.push(attribute);
                }
                else { /* todo: error */ }

                buffer.clear();
                count = reader.read_line(buffer);
            }

            self.ending_line = buffer.to_string();

            buffer.clear();
            count = reader.read_line(buffer);
        }
    }


    pub fn new() -> ItemAttributes
    {
        ItemAttributes
        {
            ending_line: String::from("}"),
            items: vec![],
            starting_line: String::from("{")
        }
    }


    pub fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        if !self.items.is_empty()
        {
            writer.write("{".as_bytes());
            writer.write("\n".as_bytes());

            for item in &self.items
            {
                item.write_to(writer);
            }

            writer.write("}".as_bytes());
            writer.write("\n".as_bytes());
        }
    }
}


#[cfg(test)]
mod test
{
    use crate::item_attributes::ItemAttributes;


    #[test]
    fn test_ending_line()
    {
        let cases = vec![
            (true,  "}"      ),
            (true,  "  }"    ),
            (true,  "}  ",   ),
            (true,  "  }  "  ),
            (true,  "}\r"    ),
            (true,  "  }\r"  ),
            (true,  "}  \r"  ),
            (true,  "  }  \r"),
            (false, ""       ),
            (false, "  "     ),
            (false, "{"      )
            ];

        for case in &cases
        {
            assert_eq!(
                ItemAttributes::is_ending_line(case.1),
                case.0,
                "erroneous case = {}", case.1
                )
        }
    }


    #[test]
    fn test_starting_line()
    {
        let cases = vec![
            (true,  "{"      ),
            (true,  "  {"    ),
            (true,  "{  ",   ),
            (true,  "  {  "  ),
            (true,  "{\r"    ),
            (true,  "  {\r"  ),
            (true,  "{  \r"  ),
            (true,  "  {  \r"),
            (false, ""       ),
            (false, "  "     ),
            (false, "}"      )
        ];

        for case in &cases
        {
            assert_eq!(
                ItemAttributes::is_starting_line(case.1),
                case.0,
                "erroneous case = {}", case.1
                )
        }
    }
}