use crate::sch::design::Design;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::ebom_op::key::Key;
use crate::ebom_op::entry::Entry;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufWriter};
use crate::output::{print_file_op, println_result};
use crate::prj::project::Project;
use crate::cfg::config_args::ConfigArgs;


#[derive(Debug, StructOpt)]
pub struct EbomSubcommand
{
    #[structopt(flatten)]
    config_args: ConfigArgs,


    #[structopt(parse(from_os_str))]
    /// Schematic input files
    files: Vec<PathBuf>,


    #[structopt(long="output", short="o", required=true)]
    /// EBOM output file
    output: PathBuf,


    #[structopt(long="project", short="p")]
    /// An optional project file
    project: Option<PathBuf>
}


impl EbomSubcommand
{
    pub fn execute(&self,) -> Result<(),Box<dyn std::error::Error>>
    {
        let config = self.config_args.load_config()?;
        let project = Project::load(self.project.as_ref())?;

        let mut files: Vec<PathBuf> = Vec::new();
        files.extend(project.schematics());
        files.extend_from_slice(&self.files);

        let schematics = Design::create(&files)?;

        let components = schematics.components();

        let pairs = components.iter()
            .flat_map(|c| Key::create(c).and_then(|k| Some((k, c))))
            .collect::<Vec<_>>();

        let unique_refdes = pairs.iter()
            .map(|p| &p.0)
            .collect::<HashSet<_>>();

        let numbers = pairs.iter()
            .map(|p| p.0.scd())
            .collect::<HashSet<_>>();

        let drawings = numbers.into_iter()
            .map(|s| (s, config.load_drawing(s).unwrap()))
            .collect::<HashMap<_,_>>();


        let entries = unique_refdes.iter()
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

        self.write_bom(&entries)?;

        Ok(())
    }


    /// Write EBOM to the output file
    fn write_bom(&self, entries: &[Entry]) -> std::io::Result<()>
    {
        print_file_op("Writing", &self.output);

        let result = self.write_bom_inner(entries);

        println_result(&result);

        result
    }


    /// Write EBOM to the output file "try block"
    fn write_bom_inner(&self, entries: &[Entry]) -> std::io::Result<()>
    {
        let file = File::create(&self.output)?;
        let mut writer = BufWriter::new(file);

        for (index, entry) in entries.iter().enumerate()
        {
            entry.write(&mut writer, index + 1)?;
        }

        Ok(())
    }
}
