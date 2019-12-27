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
use crate::schematic_net::SchematicNet;
use crate::schematic_bus::SchematicBus;
use crate::schematic_path::SchematicPath;
use crate::schematic_pin::SchematicPin;
use crate::{schematic_arc, schematic_complex, schematic_box, schematic_line, schematic_net, schematic_bus, schematic_circle, schematic_text, schematic_path, schematic_pin};


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
                schematic_arc::CODE => Ok(Box::new(SchematicArc::create(params))),
                schematic_complex::CODE => Ok(Box::new(SchematicComplex::create(params))),
                schematic_box::CODE => Ok(Box::new(SchematicBox::create(params))),
                schematic_path::CODE => Ok(Box::new(SchematicPath::create(params))),
                schematic_line::CODE => Ok(Box::new(SchematicLine::create(params))),
                schematic_net::CODE => Ok(Box::new(SchematicNet::create(params))),
                schematic_pin::CODE => Ok(Box::new(SchematicPin::create(params))),
                schematic_bus::CODE => Ok(Box::new(SchematicBus::create(params))),
                schematic_circle::CODE => Ok(Box::new(SchematicCircle::create(params))),
                schematic_text::CODE => Ok(Box::new(SchematicText::create(params, &mut reader).unwrap())),
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
        for item in &self.items
        {
            item.write_to(writer);
        }
    }
}
