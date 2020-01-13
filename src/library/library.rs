use std::path::PathBuf;
use std::error::Error;
use crate::library::loadable::Loadable;
use crate::output::{print_file_op, println_success};
use serde_derive::Deserialize;



#[derive(Deserialize)]
pub struct Library
{
    paths: Vec<PathBuf>
}


impl Library
{
    /// Obtain a item from the library
    ///
    /// # Arguments
    ///
    /// * `name` - The filename of the item including extension
    ///
    pub fn load_item<T: Loadable>(&self, name: &str) -> Result<T,Box<dyn Error>>
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

        panic!()
    }
}


#[cfg(test)]
mod test
{
    // use super::*;


    #[test]
    fn test_load()
    {
        // let mut library = Library::new().unwrap();

        //let symbol = library.load_item("ech-capacitor-non-1.sym").unwrap();
    }
}
