use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::reader::ItemReader;
use regex::Regex;
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


    fn into_text(&self) -> Option<&Text> { Some(self) }


    fn write_to(&self, writer: &mut Box<dyn Write>) -> std::io::Result<()>
    {
        self.params.write_to(writer)?;

        for line in &self.lines
        {
            writer.write(line.trim_end().as_bytes())?;
            writer.write("\n".as_bytes())?;
        }

        Ok(())
    }
}


impl Text
{
    pub fn attribute_name(&self) -> Option<String>
    {
        parse(&self.lines[0]).0.and_then(|n| Some(String::from(n)))
    }


    pub fn attribute_value(&self) -> Option<String>
    {
        let input = &self.lines.join("\n");

        parse(input).1.and_then(|v| Some(String::from(v.trim_end())))
    }


    pub fn set_attribute_value<T: ToString>(&mut self, value: T)
    {
        self.lines = build(self.attribute_name().unwrap(), value);
    }


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


fn build<T: ToString,  U: ToString>(name: T, value: U) -> Vec<String>
{
    let a = format!("{}={}", name.to_string(), value.to_string());

    a.lines().map(|s| s.to_string()).collect()
}


fn parse(input: &str) -> (Option<&str>, Option<&str>)
{
    lazy_static!
    {
            static ref REGEX: Regex = Regex::new(r"^(.+?)=(?s)(.*)").unwrap();
    }

    match REGEX.captures(input)
    {
        None => (None, None),
        Some(c) =>
            (
                c.get(1).and_then(|n| Some(n.as_str())),
                c.get(2).and_then(|v| Some(v.as_str()))
            )
    }
}


#[cfg(test)]
mod test
{
    use crate::sch::text::parse;

    #[test]
    fn test_parse()
    {
        let cases = vec!
        [
            ( "name=value",           (Some("name"), Some("value")) ),
            ( "name=value1\nvalue2",  (Some("name"), Some("value1\nvalue2")) ),
            ( "hello world",          (None, None) )
        ];

        for case in cases
        {
            let output = parse(case.0);

            assert_eq!(output, case.1);
        }
    }
}