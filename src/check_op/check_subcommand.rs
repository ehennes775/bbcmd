use std::path::PathBuf;
use structopt::StructOpt;
use std::convert::TryFrom;


#[derive(Debug, StructOpt)]
pub struct CheckSubcommand
{
    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl CheckSubcommand
{
    pub fn execute(&self) -> Result<(),Box<dyn std::error::Error>>
    {
        Err(Box::try_from("Check subcommand not implemented").unwrap())
    }
}