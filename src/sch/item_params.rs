use regex::Regex;
use std::io::Write;
use std::ops;
use std::ops::Index;
use std::str::FromStr;


pub struct ItemParams
{
    params : Vec<String>
}


impl FromStr for ItemParams
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        lazy_static!
        {
            // Add capability to include leading, internal, and trailing spaces
            // to provide better file comparison results.
            // The (^\s*)|(\s*$)|(\s+)|(\S+) does not capture leading and trailing as needed

            static ref REGEX: Regex = Regex::new(r"(\S+)").unwrap();
        }

        let params : Vec<String> = REGEX
            .find_iter(s)
            .map(|m| String::from(m.as_str()))
            .collect();

        Ok(ItemParams { params })
    }
}


impl ItemParams
{
    pub fn code(&self) -> &str
    {
        &self.params[0]
    }


    pub fn len(&self) -> usize
    {
        self.params.len()
    }


    pub fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        let output = &self.params.join(" ");

        writer.write(output.as_bytes());
    }
}


impl ops::Index<usize> for ItemParams
{
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.params[index]
    }
}


#[cfg(test)]
mod test
{
    use crate::sch::item_params::ItemParams;
    use std::io::Write;


    const LINES: &'static[&'static str] =
    &[
        "hello world",
        "  hello world",
        "hello world  ",
        "  hello world  ",
        "hello  world",
        "  hello  world",
        "hello  world  ",
        "  hello  world  "
    ];


    #[test]
    fn test_spacing()
    {
        for line in LINES
        {
            let params = line.parse::<ItemParams>().unwrap();

            assert_eq!(params[0], "hello");
            assert_eq!(params[1], "world");

            assert_eq!(params.code(), "hello");
            assert_eq!(params.len(), 2);
        }
    }


    #[test]
    fn test_write()
    {
        for line in LINES
        {
            let buffer: Vec<u8> = Vec::new();
            let mut writer: Box<dyn Write> = Box::new(buffer);

            let params = line.parse::<ItemParams>().unwrap();
            params.write_to(&mut writer);

            // TODO:figure out ownership here

            //let output = String::from_utf8(buffer).unwrap();
            //assert_eq!(output, *line);
        }
    }

}