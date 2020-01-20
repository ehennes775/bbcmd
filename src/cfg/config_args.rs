use structopt::StructOpt;
use std::path::PathBuf;


/// Command line arguments for setting up the configuration
#[derive(Debug, StructOpt)]
pub struct ConfigArgs
{
    #[structopt(long="config", short="c")]
    /// An optional alternate configuration file
    path : Option<PathBuf>
}