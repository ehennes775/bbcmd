use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub struct CheckSubcommand
{
    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl CheckSubcommand
{
    pub fn execute(&self) -> Result<(),(&str)>
    {
        Err("Check subcommand not implemented")
    }
}