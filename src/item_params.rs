use regex::Regex;
use std::io::Write;
use std::ops;
use std::ops::Index;
use std::str::FromStr;


pub struct ItemParams
{
    params : Vec<String>
}


impl ops::Index<usize> for ItemParams
{
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.params[index]
    }
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


#[cfg(test)]
mod test
{
    use crate::item_params::ItemParams;


    #[test]
    fn test_spacing()
    {
        let lines = vec!(
            "hello world",
            "  hello world",
            "hello world  ",
            "  hello world  "
            );

        for line in lines
        {
            let params = line.parse::<ItemParams>().unwrap();

            assert_eq!(params[0], "hello");
            assert_eq!(params[1], "world");

            assert_eq!(params.code(), "hello");
            assert_eq!(params.len(), 2);
        }
    }

}