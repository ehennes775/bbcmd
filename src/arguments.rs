use structopt::StructOpt;
use crate::check_op::check_subcommand::CheckSubcommand;
use crate::refdes_op::refdes_subcommand::RefdesSubcommand;
use crate::ebom_op::ebom_subcommand::EbomSubcommand;
use crate::cfg::config::Config;


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


    #[structopt(name="ebom")]
    /// Provides engineering bill of material (EBOM) processing functionality
    Ebom
    {
        #[structopt(flatten)]
        subcommand: EbomSubcommand
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
    pub fn execute(&self, config: Box<Config>) -> Result<(),Box<dyn std::error::Error>>
    {
        match self
        {
            Arguments::Check { subcommand }  => subcommand.execute(config),
            Arguments::Refdes { subcommand }  => subcommand.execute(config),
            Arguments::Ebom { subcommand }  => subcommand.execute(config)
        }
    }
}