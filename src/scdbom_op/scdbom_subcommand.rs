use crate::sch::design::Design;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::scdbom_op::key::Key;
use crate::scdbom_op::entry::Entry;
use std::collections::{HashSet, HashMap};
use crate::cfg::config::Config;
use std::fs::File;
use std::io::{BufWriter};
use crate::output::{print_file_op, println_result};


#[derive(Debug, StructOpt)]
pub struct ScdBomSubcommand
{
    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files: Vec<PathBuf>,


    #[structopt(long="output", short="o", required=true)]
    /// EBOM output file
    output: PathBuf,


    #[structopt(long="project", short="p")]
    /// An optional project file
    project: Option<PathBuf>
}


impl ScdBomSubcommand
{
    pub fn execute(&self, config: Box<Config>) -> Result<(),Box<dyn std::error::Error>>
    {
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
            .map(|s| (s, config.load_drawing(s).unwrap()))
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
            .collect::<Vec<_>>();

        self.write_bom(&entries);

        Ok(())
    }


    /// Write EBOM to the output file
    fn write_bom(&self, entries: &[Entry])
    {
        print_file_op("Writing", &self.output);

        let func = || -> std::io::Result<()>
        {
            let file = File::create(&self.output)?;
            let mut writer = BufWriter::new(file);

            for (index, entry) in entries.iter().enumerate()
            {
                Self::write_entry(&mut writer, index + 1, entry)?;
            }

            Ok(())
        };

        let result = func();

        println_result(&result);
    }


    /// Write a single line item to the output file
    fn write_entry<T: std::io::Write>(writer: &mut T, number: usize, entry: &Entry) -> std::io::Result<()>
    {
        write!(writer, "{:4}|", number)?;
        write!(writer, "{:16}|", entry.scd())?;
        write!(writer, "{:16}|", entry.value())?;
        write!(writer, "{}", entry.description())?;
        writeln!(writer)?;

        let mut z = entry.refdes().collect::<Vec<_>>();

        z.sort();

        let y = z.iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join(",");

        writeln!(writer, "        {}", &y)?;

        for part in entry.parts()
        {
            writeln!(writer, "        {:20}|{}", part.manufacturer(), part.part_number())?;
        }

        Ok(())
    }
}
