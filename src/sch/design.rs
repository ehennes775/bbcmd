use std::path::PathBuf;
use crate::sch::page::Page;
use std::fmt::{Debug, Formatter, Error};
use std::fs::File;
use std::io::Write;
use crate::sch::complex::Complex;
use crate::sch::item::Item;
use crate::output::{println_result, print_file_op};


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
    #[allow(dead_code)]
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


    pub fn components(&self) -> Vec<&Complex>
    {
        self.pages.iter()
            .flat_map(|page| page.items.iter())
            .map(|item| item.into_complex())
            .flat_map(|o| o)
            .collect()
    }


    pub fn components_mut(&mut self) -> Vec<&mut Complex>
    {
        self.pages.iter_mut()
            .flat_map(|page| page.items.iter_mut())
            .map(|item| item.into_complex_mut())
            .flat_map(|o| o)
            .collect()
    }


    pub fn create(files : &[PathBuf]) -> Result<Design,Box::<dyn std::error::Error>>
    {
        let pages = files.into_iter()
            .inspect(|p| print_file_op("Reading", p))
            .map(|f| Page::create(f))
            .inspect(|_r| println_result(&Ok(())))
            .collect::<Result<Vec<_>,_>>()?;

        Ok(Design { pages })
    }


    // TODO temporarily writes to a file prefixed with out_ for development
    pub fn write(&self)
    {
        for page in &self.pages
        {
            let output_filename = format!("out_{}", page.path.file_name().unwrap().to_str().unwrap());

            let file = File::create(&output_filename).unwrap();

            print!("Writing {}... ", output_filename);

            let mut output: Box<dyn Write> = Box::new(file);

            let status = page.write_to(&mut output);

            println_result(&status);
        }
    }
}
