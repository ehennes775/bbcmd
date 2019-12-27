use std::borrow::{Borrow};
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::path::PathBuf;
use crate::item_params::ItemParams;
use crate::schematic_line::SchematicLine;
use crate::schematic_item::SchematicItem;
use crate::schematic_circle::SchematicCircle;
use crate::schematic_arc::SchematicArc;
use crate::schematic_box::SchematicBox;
use crate::schematic_complex::SchematicComplex;
use crate::schematic_text::SchematicText;


pub struct SchematicPage
{
    items : Vec<Box<dyn SchematicItem>>,


    path : PathBuf
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

        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let mut count = reader.read_line(&mut buffer);

        if count.borrow().as_ref().unwrap_or_else(|_e| &0) > &0
        {
            let params = buffer.parse::<ItemParams>().unwrap();

            if params.code() == "v"
            {
                buffer.clear();
                count = reader.read_line(&mut buffer);
            }
        }

        let mut items : Vec<Box<dyn SchematicItem>> = Vec::new();

        while count.borrow().as_ref().unwrap_or_else(|_e| &0) > &1
        {
            let params = buffer.parse::<ItemParams>().unwrap();

            let item:Result<Box<dyn SchematicItem>,&str> = match params.code()
            {
                "A" => Ok(Box::new(SchematicArc::create(params))),
                "C" => Ok(Box::new(SchematicComplex::create(params))),
                "B" => Ok(Box::new(SchematicBox::create(params))),
                "L" => Ok(Box::new(SchematicLine::create(params))),
                "V" => Ok(Box::new(SchematicCircle::create(params))),
                "T" => Ok(Box::new(SchematicText::create(params, &mut reader).unwrap())),
                _ => Err("")
            };

            match item
            {
                Ok(i) => items.push(i),
                Err(_e) => println!("ignore = {}", buffer)
            }

            buffer.clear();
            count = reader.read_line(&mut buffer);
        }

        match count
        {
            Err(_e) => println!("{}", "Failure"),
            Ok(_t) => println!("{}", "Success")
        }

        Ok(SchematicPage
        {
            items,
            path : path.clone()
        })
    }


    pub fn write_to(&self, writer: &mut Box<dyn Write>)
    {
        //println!("path = {}", self.path.as_os_str());

        for item in &self.items
        {
            item.write_to(writer);
        }
    }
}
