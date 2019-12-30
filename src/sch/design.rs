use std::path::PathBuf;
use crate::sch::page::Page;
use std::fmt::{Debug, Formatter, Error};
use crate::sch::item::Item;
use crate::sch;
use std::convert::{TryInto, TryFrom};
use crate::sch::text::Text;
use crate::sch::complex::Complex;


pub const NAME: &str = "Design";


pub struct Design
{
    pub pages : Vec<Page>
}


impl Debug for Design
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "{} {{ pages={} }}", NAME, &self.pages.len())
    }
}


impl Design
{
    pub fn attributes(&self)
    {
//        let x = self.pages
//            .iter()
//            .map(|p| p.items.iter())
//            .flatten()
//            .map(|i| i.into_text())
//            .flat_map(|x| x)
//            .filter(|x| x.attribute_name().is_some())
//            .map(|x| x.attribute_value());
//
//        for y in x
//        {
//            println!("{:?}", y);
//        }
    }


    pub fn create(files : &[PathBuf]) -> Result<Design,&str>
    {
        let result : Result<Vec<Page>,_> = files
            .iter()
            .map(|f| Page::create(f))
            .collect();

        match result
        {
            Err(t) => Err(t),
            Ok(pages) => Ok(Design { pages })
        }
    }
}