use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use std::str::FromStr;
use crate::sch::schematic_reader::SchematicReader;
use crate::sch::schematic_version;


pub struct SchematicPage
{
    items : Vec<Box<dyn SchematicItem>>,


    path : PathBuf,


    version : ItemParams
}


impl SchematicPage
{
    pub fn create(path : &PathBuf) -> Result<SchematicPage,&str>
    {
        let file = match File::open(path)
        {
            Err(_e) => return Err("nope"),
            Ok(t) => t
        };

        let mut reader = SchematicReader::new(file);
        let mut params = reader.x2().unwrap();

        let version = if params.code() == schematic_version::CODE
        {
            let version = params;
            params = reader.x2().unwrap();
            version
        }
        else
        {
            ItemParams::from_str("hello").unwrap() // a default version
        };

        let mut items : Vec<Box<dyn SchematicItem>> = vec![];

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

        Ok(SchematicPage
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
