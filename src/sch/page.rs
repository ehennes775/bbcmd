use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use std::str::FromStr;
use crate::sch::reader::Reader;
use crate::sch::version;
use std::fmt::{Formatter, Debug, Error};
use crate::library::loadable::Loadable;
use crate::sch::complex::Complex;


pub const NAME: &str = "Page";


pub struct Page
{
    pub items : Vec<Box<dyn Item>>,

    pub path : PathBuf,

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
    pub fn components(&self) -> Vec<&Complex>
    {
        self.items.iter()
            .map(|item| item.into_complex())
            .flat_map(|o| o)
            .collect()
    }


    pub fn create<T: AsRef<Path>>(path: T) -> Result<Page,Box<dyn std::error::Error>>
    {
        let file =  File::open(&path)?;

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
            path : PathBuf::from(path.as_ref()),
            version
        })
    }


    pub fn id(&self) -> String
    {
        String::from(self.path.file_name().unwrap().to_str().unwrap())
    }


    pub fn write_to(&self, writer: &mut Box<dyn Write>) -> std::io::Result<()>
    {
        self.version.write_to(writer)?;

        for item in &self.items
        {
            item.write_to(writer)?;
        }

        Ok(())
    }
}


impl Loadable for Page
{
    fn load<T: AsRef<Path>>(path: T) -> Result<Box<Self>, Box<dyn std::error::Error>>
    {
        let page = Page::create(path)?;

        Ok(Box::new(page))
    }
}