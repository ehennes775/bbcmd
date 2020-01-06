use crate::scd::drawing::Drawing;
use crate::scd::part::Part;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use regex::Error;
use std::collections::HashMap;


pub struct Library
{
    cache: HashMap<String, Drawing>,


    path: PathBuf,
}


impl Library
{
    pub fn new() -> Result<Library,Box<dyn std::error::Error>>
    {
        Ok(Library
        {
            cache: HashMap::new(),
            path: PathBuf::from("/home/ehennes/Projects/edalib/scd")
        })
    }



//    pub fn get_drawing(&mut self, number: &str) -> Result<&Drawing,()>
//    {
//        match self.cache.get(number)
//        {
//            Some(drawing) => Ok(drawing),
//            None =>
//                {
//                    let result = self.load_drawing(number);
//
//                    if let Ok(drawing) = result
//                    {
//                        self.cache.insert(String::from(number), drawing);
//
//                        Ok(&drawing)
//                    }
//                    else
//                    {
//                        Err(())
//                    }
//                }
//        }
//    }


    pub fn load_drawing(&self, number: &str) -> Result<Drawing,()>
    {
        let filename = format!("{}.json", number);

        let mut path = self.path.clone();
        path.push(filename);

        println!("{:?}", path);

        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);

        let u: Drawing = serde_json::from_reader(reader).unwrap();

        Ok(u)
    }
}


#[cfg(test)]
mod test
{
    use crate::scd::library::Library;


    #[test]
    fn test_load()
    {
        let library = Library::new().unwrap();

        let drawing = library.load_drawing("100001").unwrap();

        let parts = drawing.find_parts("24 V").collect::<Vec<_>>();

        panic!("{:?}", parts);
    }
}


