use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::str::FromStr;
use crate::sch::reader::Reader;
use crate::sch::version;
use std::fmt::{Formatter, Debug, Error};


pub const NAME: &str = "Page";


pub struct Page
{
    pub items : Vec<Box<dyn Item>>,

    path : PathBuf,

    version : ItemParams
}


impl Debug for Page
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        match self.items.len()
        {
            1usize => write!(f, "{} {{ {:?} }}", NAME, &self.items[0]),
            count => write!(f, "{} {{ items={} }}", NAME, count)
        }
    }
}


impl Page
{
    pub fn create(path : &PathBuf) -> Result<Page,&str>
    {
        let file = match File::open(path)
        {
            Err(_e) => return Err("nope"),
            Ok(t) => t
        };

        let mut reader = Reader::new(file);
        let mut params = reader.x2().unwrap();

        let version = if params.code() == version::CODE
        {
            let version = params;
            params = reader.x2().unwrap();
            version
        }
        else
        {
            ItemParams::from_str("hello").unwrap() // a default version
        };

        let mut items : Vec<Box<dyn Item>> = vec![];

        while reader.count > 1
        {
            reader.x1();

            if params.len() > 0
            {
                match reader.x3(params)
                {
                    Err(_e) => println!("ignore = {}", reader.lookahead()),
                    Ok(i) => items.push(i)
                }
            }

            params = reader.lookahead().parse::<ItemParams>().unwrap();
        }

        Ok(Page
        {
            items,
            path : path.clone(),
            version
        })
    }


    pub fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        for item in &self.items
        {
            item.write_to(writer);

            if let Some(attributes) = item.attributes()
            {
            }
        }
    }
}
