use std::io::{BufReader, Read, BufRead};
use crate::sch::item_params::ItemParams;
use crate::sch::schematic_item::SchematicItem;
use crate::sch::{schematic_arc, schematic_complex, schematic_box, schematic_text, schematic_circle, schematic_bus, schematic_pin, schematic_net, schematic_line, schematic_path};
use crate::sch::schematic_arc::SchematicArc;
use crate::sch::schematic_text::SchematicText;
use crate::sch::schematic_circle::SchematicCircle;
use crate::sch::schematic_bus::SchematicBus;
use crate::sch::schematic_pin::SchematicPin;
use crate::sch::schematic_net::SchematicNet;
use crate::sch::schematic_line::SchematicLine;
use crate::sch::schematic_path::SchematicPath;
use crate::sch::schematic_box::SchematicBox;
use crate::sch::schematic_complex::SchematicComplex;


pub trait ItemReader
{
    fn lookahead(&mut self) -> String;
    fn x9(&mut self) -> String;

    fn read_lines(&mut self, count: usize) -> Result<Vec<String>,i32>;
}


pub struct SchematicReader<T>
{
    // temporary public

    pub buffer : String,

    pub count : usize,

    pub reader : BufReader<T>,
}



impl<T: Read> ItemReader for SchematicReader<T>
{
    fn lookahead(&mut self) -> String { self.buffer.to_string() }

    fn x9(&mut self) -> String
    {
        self.buffer.clear();
        self.count = self.reader.read_line(&mut self.buffer).unwrap();

        self.buffer.to_string()
    }


    fn read_lines(&mut self, count: usize) -> Result<Vec<String>,i32>
    {
        let mut lines = vec![];

        for count in 0..count
        {
            lines.push(self.buffer.to_string());

            self.buffer.clear();
            let count2 = self.reader.read_line(&mut self.buffer).unwrap();
        }

        Ok(lines)
    }
}


impl<T: Read> SchematicReader<T>
{
    pub fn new(inner: T) -> SchematicReader<T>
    {
        let reader = SchematicReader
        {
            buffer: String::new(),
            count : 0,
            reader : BufReader::new(inner)
        };

        reader
    }


    pub fn lookahead(&self) -> String
    {
        self.buffer.to_string()
    }


    pub fn x1(&mut self) -> String
    {
        self.buffer.clear();
        self.count = self.reader.read_line(&mut self.buffer).unwrap();

        self.buffer.to_string()
    }


    pub fn x2(&mut self) -> Result<ItemParams,i32>
    {
        Ok(self.x1().parse::<ItemParams>().unwrap())
    }


    pub fn x3(&mut self, params: ItemParams) -> Result<Box<dyn SchematicItem>, i32>
    {
        match params.code()
        {
            schematic_arc::CODE =>
                {
                    Ok(Box::new(SchematicArc::create(params)))
                },
            schematic_complex::CODE =>
                {
                    Ok(Box::new(SchematicComplex::create(params, self)))
                },
            schematic_box::CODE =>
                {
                    Ok(Box::new(SchematicBox::create(params)))
                },
            schematic_path::CODE =>
                {
                    Ok(Box::new(SchematicPath::create(params, self).unwrap()))
                },
            schematic_line::CODE =>
                {
                    Ok(Box::new(SchematicLine::create(params)))
                },
            schematic_net::CODE =>
                {
                    Ok(Box::new(SchematicNet::create(params, self)))
                },
            schematic_pin::CODE =>
                {
                    Ok(Box::new(SchematicPin::create(params, self)))
                },
            schematic_bus::CODE =>
                {
                    Ok(Box::new(SchematicBus::create(params, self)))
                },
            schematic_circle::CODE =>
                {
                    Ok(Box::new(SchematicCircle::create(params)))
                },
            schematic_text::CODE =>
                {
                    Ok(Box::new(SchematicText::create(params, self).unwrap()))
                },
            _ =>
                {
                    Err(10)
                }
        }
    }
}