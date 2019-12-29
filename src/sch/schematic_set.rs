use std::path::PathBuf;
use crate::sch::page::Page;


pub struct SchematicSet
{
    pages : Vec<Page>
}


impl SchematicSet
{
    pub fn create(files : &[PathBuf]) -> Result<SchematicSet,&str>
    {
        let result : Result<Vec<Page>,_> = files
            .iter()
            .map(|f| Page::create(f))
            .collect();

        match result
        {
            Err(t) => Err(t),
            Ok(pages) => Ok(SchematicSet { pages })
        }
    }
}