use std::path::PathBuf;
use crate::sch::page::Page;
use std::fmt::{Debug, Formatter, Error};
use std::fs::File;
use std::io::Write;


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
    pub fn dump(&self)
    {
        println!("{:?}", self);

        for page in &self.pages
        {
            println!("    {:?}", page);

            for item in &page.items
            {
                println!("        {:?}", item);

                if let Some(complex) = item.into_complex()
                {
                    for attribute in &complex.attributes.items
                    {
                        println!("            {:?}", attribute);
                    }
                }
            }
        }
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

    // TODO temporarily writes to a file prefixed with out_ for development
    pub fn write(&self) -> std::io::Result<()>
    {
        for page in &self.pages
        {
            let output_filename = format!("out_{}", page.path.file_name().unwrap().to_str().unwrap());

            let file = File::create(&output_filename).unwrap();

            println!("Writing {:?}", output_filename);

            let mut output: Box<dyn Write> = Box::new(file);

            page.write_to(&mut output)?;
        }

        Ok(())
    }
}