use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub struct RefdesSubcommand
{
    #[structopt(long="renumber", short="n")]
    /// Renumber reference designators
    renumber :  bool,


    #[structopt(long="reset", short="r")]
    /// Reset reference designators
    reset : bool,


    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl RefdesSubcommand
{
    pub fn execute(&self) -> Result<(),(&str)>
    {
        Ok(())
    }
}