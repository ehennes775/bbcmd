use regex::Regex;
use std::io::{Write};
use std::str::FromStr;
use crate::sch::text::Text;
use crate::sch::item_params::ItemParams;
use crate::sch::text;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;


pub struct ItemAttributes
{
    ending_line : String,

    items : Vec<Text>,

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


    pub fn read_from(reader: &mut impl ItemReader) -> Result<ItemAttributes,i32>
    {
        let mut ending_line = String::from("}");
        let mut items= vec![];
        let mut starting_line = String::from("{");

        let mut lookahead = reader.lookahead();

        if Self::is_starting_line(&lookahead)
        {
            starting_line = lookahead;

            lookahead = reader.x9();

            while !Self::is_ending_line(&lookahead)
            {
                let params = ItemParams::from_str(&lookahead).unwrap();

                if params.code() == text::CODE
                {
                    let attribute = Text::create(params, reader).unwrap();

                    items.push(attribute);
                }
                else { /* todo: error */ }

                lookahead = reader.x9();
            }

            ending_line = lookahead;

            reader.x9();
        }

        Ok(ItemAttributes
        {
            ending_line,
            items,
            starting_line
        })
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
    use crate::sch::item_attributes::ItemAttributes;


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