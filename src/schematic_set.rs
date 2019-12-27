use std::path::PathBuf;
use crate::schematic_page::SchematicPage;


pub struct SchematicSet
{
    pages : Vec<SchematicPage>
}


impl SchematicSet
{
    pub fn create(files : &[PathBuf]) -> Result<SchematicSet,&str>
    {
        let result : Result<Vec<SchematicPage>,_> = files
            .iter()
            .map(|f| SchematicPage::create(f))
            .collect();

        match result
        {
            Err(t) => Err(t),
            Ok(pages) => Ok(SchematicSet { pages })
        }
    }
}