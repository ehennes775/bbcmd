use crate::sch::page::Page;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::error::Error;
use crate::library::loadable::Loadable;
use crate::output::{print_file_op, println_result, println_success};


pub struct Library<T>
{
    cache: HashMap<String, T>,

    paths: Vec<PathBuf>
}


impl<T: Loadable> Library<T>
{
    pub fn new() -> Result<Library<T>,Box<dyn std::error::Error>>
    {
        Ok(Library
        {
            cache: HashMap::new(),
            paths: vec![PathBuf::from("/home/ehennes/Projects/edalib/scd/scd")]
        })
    }


    /// Obtain a item from the library
    ///
    /// # Arguments
    ///
    /// * `name` - The filename of the item including extension
    ///
    pub fn load_item(&mut self, name: &str) -> Result<T,Box<dyn Error>>
    {
        print_file_op("Reading", &PathBuf::from(name));

        for path in &self.paths
        {
            let mut p = path.clone();
            p.push(name);

            let item = T::load(&p);

            if let Ok(i) = item
            {
                println_success("Ok");

                return Ok(*i);
            }
        }

        panic!(); //Ok(item.unwrap());
    }
}


#[cfg(test)]
mod test
{
    use crate::library::library::Library;
    use crate::sch::page::Page;


    #[test]
    fn test_load()
    {
        let mut library = Library::<Page>::new().unwrap();

        let symbol = library.load_item("ech-capacitor-non-1.sym").unwrap();
    }
}
