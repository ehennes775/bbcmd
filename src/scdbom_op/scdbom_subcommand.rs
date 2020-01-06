use crate::sch::design::Design;
use crate::sch::complex::Complex;
use crate::sch::text::Text;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;
use std::io::stdout;
use std::fs::File;
use crate::output::{println_result, print_file_op};
use crate::sch::page::Page;
use crate::scd::drawing::Drawing;
use crate::scdbom_op::key::Key;
use crate::scdbom_op::entry::Entry;
use std::collections::{HashSet, HashMap};


#[derive(Debug, StructOpt)]
pub struct ScdBomSubcommand
{
    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl ScdBomSubcommand
{
    pub fn execute(&self) -> Result<(),Box<dyn std::error::Error>>
    {
        let mut symbol_library = crate::library::library::Library::<Page>::new()?;
        let mut scd_library = crate::library::library::Library::<Drawing>::new()?;

        let schematics = Design::create(&self.files)?;

        let components = schematics.components();

        let pairs = components.iter()
            .flat_map(|c| Key::create(c).and_then(|k| Some((k, c))))
            .collect::<Vec<_>>();

        let keys = pairs.iter()
            .map(|p| &p.0)
            .collect::<HashSet<_>>();

        let numbers = pairs.iter()
            .map(|p| p.0.scd())
            .collect::<HashSet<_>>();

        let drawings = numbers.into_iter()
            .map(|s| (s, scd_library.load_item(s).unwrap()))
            .collect::<HashMap<_,_>>();


        let entries = keys.iter()
            .map(|key|
                {
                    let c = pairs.iter()
                        .filter(|pair| pair.0 == **key)
                        .map(|pair| *pair.1)
                        .collect::<Vec<_>>();

                    let drawing = &drawings[key.scd()];
                    Entry::new(&key, drawing, c)
                })
            ; //.collect::<Vec<_>>();


        for (index, entry) in entries.enumerate()
        {
            print!("{:4}|", index);
            print!("{:16}|", entry.scd());
            print!("{:16}|", entry.value());
            print!("{}", entry.description());
            println!();

            let mut z = entry.refdes().collect::<Vec<_>>();

            z.sort();

            let y = z.iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(",");

            println!("        {}", &y);

            for part in entry.parts()
            {
                println!("        {:20}|{}", part.manufacturer(), part.part_number());
            }
        }

        Ok(())
    }
}
