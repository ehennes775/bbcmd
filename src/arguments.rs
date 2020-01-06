use structopt::StructOpt;
use crate::check_op::check_subcommand::CheckSubcommand;
use crate::refdes_op::refdes_subcommand::RefdesSubcommand;
use crate::scdbom_op::scdbom_subcommand::ScdBomSubcommand;


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
    },


    #[structopt(name="scdbom")]
    /// Provides REFDES processing functionality
    ScdBom
    {
        #[structopt(flatten)]
        subcommand: ScdBomSubcommand
    }
}


impl Arguments
{
    pub fn execute(&self) -> Result<(),Box<dyn std::error::Error>>
    {
        match self
        {
            Arguments::Check { subcommand }  => subcommand.execute(),
            Arguments::Refdes { subcommand }  => subcommand.execute(),
            Arguments::ScdBom { subcommand }  => subcommand.execute()
        }
    }
}