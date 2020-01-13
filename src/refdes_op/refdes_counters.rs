use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, Write, Read, BufRead, BufReader};
use crate::refdes_op::refdes::Refdes;
use std::cmp::max;
use crate::output::{println_result, print_file_op};
use std::fs::File;
use std::path::PathBuf;


pub struct RefdesCounters
{
    counters: HashMap<String,i32>
}


impl RefdesCounters
{
    pub fn assign_number(&mut self, refdes: &mut Refdes)
    {
        let prefix = &refdes.prefix;

        let number = self.counters.get(prefix).unwrap_or(&0) + 1;

        refdes.assign(number);

        self.push(refdes);
    }


    pub fn new() -> RefdesCounters
    {
        RefdesCounters { counters: HashMap::new() }
    }


    pub fn push(&mut self, refdes: &Refdes)
    {
        if let Some(number1) = refdes.number
        {
            let number2 = *self.counters.get(&refdes.prefix).unwrap_or(&0);

            self.counters.insert(
                String::from(&refdes.prefix),
                max(number1, number2)
                );
        }
    }


    pub fn read_from<T: Read>(&mut self, reader: T) -> Result<(),Error>
    {
        let buffered = BufReader::new(reader);

        for line in buffered.lines()
        {
            match parse_line(&line?)
            {
                ParseEvent::Blank => {},
                ParseEvent::Comment => {},
                ParseEvent::Err(_) => {},
                ParseEvent::Refdes(r) =>
                    {
                        &self.push(&r);
                    },
            }
        }

        Ok(())
    }


    pub fn read_from_file(&mut self, path: &PathBuf) -> Result<(),Error>
    {
        print_file_op("Reading REFDES counter file", &path);

        let file = File::open(&path).unwrap();
        let result = self.read_from(file);

        println_result(&result);

        result
    }


    pub fn write_to<T: Write>(&self, mut writer: T) -> Result<(),Error>
    {
        for line in &BOILERPLATE
        {
            writeln!(&mut writer, "{}", line)?;
        }

        let mut refdes_list = self.counters.iter()
            .map(|c| Refdes::new(c.0, *c.1, ""))
            .collect::<Vec<_>>();

        refdes_list.sort();

        for refdes in &refdes_list
        {
            writeln!(&mut writer, "{}", refdes.to_string())?;
        }

        Ok(())
    }


    pub fn write_to_file(&self, path: &PathBuf) -> Result<(),Error>
    {
        print_file_op("Writing REFDES counter file", &path);

        let file = File::create(&path).unwrap();
        let result = self.write_to(file);

        println_result(&result);

        result
    }
}


const BOILERPLATE: [&str; 4] =
[
    "# REFDES counter file",
    "# Contains the greatest number, by REFDES prefix, present in schematics",
    "# Updated by the bbcmd command line utility",
    ""
];


enum ParseEvent
{
    Blank,
    Comment,
    Err(String),
    Refdes(Refdes)
}


fn parse_line(line: &str) -> ParseEvent
{
    lazy_static!
    {
        static ref REGEX: Regex = Regex::new(r"^(#.*)|(\S+)").unwrap();
    }

    if let Some(capture) = REGEX.captures(line)
    {
        if capture.get(1).is_some()
        {
            ParseEvent::Comment
        }
        else if let Some(t) = capture.get(2)
        {
            match t.as_str().trim().parse::<Refdes>()
            {
                Err(_e) =>
                    {
                        ParseEvent::Err(format!("Illegal REFDES"))
                    },
                Ok(t) => ParseEvent::Refdes(t)
            }
        }
        else
        {
            ParseEvent::Blank
        }
    }
    else
    {
        ParseEvent::Blank
    }
}


#[cfg(test)]
mod test
{
    use crate::refdes_op::refdes_counters::{parse_line, ParseEvent};


    #[test]
    fn test_parse_blank()
    {
        let cases = vec!
        [
            "",
            "    "
        ];

        for case in cases
        {
            match parse_line(case)
            {
                ParseEvent::Blank => {},
                ParseEvent::Comment | ParseEvent::Refdes(_)=>
                    {
                        panic!("Failing test case `{}`", case)
                    }
                ParseEvent::Err(e) =>
                    {
                        panic!("Failing test case `{}` with {}", case, e)
                    }
            }
        }
    }


    #[test]
    fn test_parse_comment()
    {
        let cases = vec!
        [
            "#",
            "# hello",
        ];

        for case in cases
        {
            match parse_line(case)
            {
                ParseEvent::Blank | ParseEvent::Refdes(_)=>
                    {
                        panic!("Failing test case = `{}`", case)
                    },
                ParseEvent::Comment => {}
                ParseEvent::Err(e) =>
                    {
                        panic!("Failing test case `{}` with {}", case, e)
                    }
            }
        }
    }


    #[test]
    fn test_parse_error()
    {
        let cases = vec!
        [
            "    #",
            "hello",
            "    hello",
            "hello    ",
            "    hello    ",
        ];

        for case in cases
        {
            match parse_line(case)
            {
                ParseEvent::Blank | ParseEvent::Comment | ParseEvent::Refdes(_)=>
                    {
                        panic!("Failing test case = `{}`", case)
                    },
                ParseEvent::Err(_) => {}
            }
        }
    }


    #[test]
    fn test_parse_spacing()
    {
        let cases = vec!
        [
            "C1",
            "    C1",
            "C1    ",
            "    C1    ",
        ];

        for case in cases
        {
            match parse_line(case)
            {
                ParseEvent::Blank | ParseEvent::Comment =>
                    {
                        panic!("Failing test case = `{}`", case)
                    },
                ParseEvent::Err(e) =>
                    {
                        panic!("Failing test case `{}` with {}", case, e)
                    }
                ParseEvent::Refdes(r) =>
                    {
                        assert_eq!(r.to_string(), "C1");
                    }
            }
        }
    }
}