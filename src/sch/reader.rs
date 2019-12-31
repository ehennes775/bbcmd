use std::io::{BufReader, Read, BufRead};
use crate::sch::item_params::ItemParams;
use crate::sch::item::Item;
use crate::sch::{arc, complex, r#box, text, circle, bus, pin, net, line, path};
use crate::sch::arc::Arc;
use crate::sch::text::Text;
use crate::sch::circle::Circle;
use crate::sch::bus::Bus;
use crate::sch::pin::Pin;
use crate::sch::net::Net;
use crate::sch::line::Line;
use crate::sch::path::Path;
use crate::sch::r#box::Box;
use crate::sch::complex::Complex;


pub trait ItemReader
{
    fn lookahead(&mut self) -> String;
    fn x9(&mut self) -> String;

    fn read_lines(&mut self, count: usize) -> Result<Vec<String>,i32>;
}


pub struct Reader<T>
{
    // temporary public

    pub buffer : String,

    pub count : usize,

    pub reader : BufReader<T>,
}



impl<T: Read> ItemReader for Reader<T>
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

        for _count in 0..count
        {
            lines.push(self.buffer.to_string());

            self.buffer.clear();
            let _count2 = self.reader.read_line(&mut self.buffer).unwrap();
        }

        Ok(lines)
    }
}


impl<T: Read> Reader<T>
{
    pub fn new(inner: T) -> Reader<T>
    {
        let reader = Reader
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


    pub fn x3(&mut self, params: ItemParams) -> Result<std::boxed::Box<dyn Item>, i32>
    {
        match params.code()
        {
            arc::CODE =>
                {
                    Ok(std::boxed::Box::new(Arc::create(params)))
                },
            complex::CODE =>
                {
                    Ok(std::boxed::Box::new(Complex::create(params, self)))
                },
            r#box::CODE =>
                {
                    Ok(std::boxed::Box::new(Box::create(params)))
                },
            path::CODE =>
                {
                    Ok(std::boxed::Box::new(Path::create(params, self).unwrap()))
                },
            line::CODE =>
                {
                    Ok(std::boxed::Box::new(Line::create(params)))
                },
            net::CODE =>
                {
                    Ok(std::boxed::Box::new(Net::create(params, self)))
                },
            pin::CODE =>
                {
                    Ok(std::boxed::Box::new(Pin::create(params, self)))
                },
            bus::CODE =>
                {
                    Ok(std::boxed::Box::new(Bus::create(params, self)))
                },
            circle::CODE =>
                {
                    Ok(std::boxed::Box::new(Circle::create(params)))
                },
            text::CODE =>
                {
                    Ok(std::boxed::Box::new(Text::create(params, self).unwrap()))
                },
            _ =>
                {
                    Err(10)
                }
        }
    }
}