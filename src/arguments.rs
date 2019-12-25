use structopt::StructOpt;
use crate::check::check_subcommand::CheckSubcommand;
use crate::refdes::refdes_subcommand::RefdesSubcommand;


#[derive(Debug, StructOpt)]
#[structopt(about="Command line utility for Lepton-EDA")]
pub enum Arguments
{
    #[structopt(name="check")]
    /// Provides schematic design rule checks
    Check
    {
        #[structopt(flatten)]
        subcommand: CheckSubcommand
    },


    #[structopt(name="refdes")]
    /// Provides REFDES processing functionality
    Refdes
    {
        #[structopt(flatten)]
        subcommand: RefdesSubcommand
    }
}


impl Arguments
{
    pub fn execute(&self) -> Result<(),(&str)>
    {
        match self
        {
            Arguments::Check { subcommand }  => subcommand.execute(),
            Arguments::Refdes { subcommand }  => subcommand.execute()
        }
    }
}