use std::path::PathBuf;
use structopt::StructOpt;
use crate::sch::design::Design;
use crate::check_op::checks::unassigned_refdes::UnassignedRefdes;
use crate::check_op::check::Check;
use crate::check_op::error::Error;
use crate::check_op::checks::invalid_refdes::InvalidRefdes;
use crate::cfg::config_args::ConfigArgs;


#[derive(Debug, StructOpt)]
pub struct CheckSubcommand
{
    #[structopt(flatten)]
    config_args: ConfigArgs,


    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl CheckSubcommand
{
    pub fn execute(&self) -> Result<(),Box<dyn std::error::Error>>
    {
        let _config = self.config_args.load_config()?;

        let checks: Vec<fn(&Design)->Vec<Box<dyn Error>>> = vec!
        [
            InvalidRefdes::check,
            UnassignedRefdes::check
        ];

        let design = Design::create(&self.files)?;

        let errors = checks.iter()
            .flat_map(|check| check(&design))
            .collect::<Vec<_>>();

        for error in errors
        {
            println!("{}", error);
        }

        Ok(())
    }
}
