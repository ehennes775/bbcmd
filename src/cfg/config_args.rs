use crate::cfg::config;
use crate::cfg::config::Config;
use structopt::StructOpt;
use std::path::PathBuf;
use std::env;


/// Command line arguments for setting up the configuration
#[derive(Debug, StructOpt)]
pub struct ConfigArgs
{
    #[structopt(long="config", short="c")]
    /// An optional alternate configuration file
    path : Option<PathBuf>
}


impl ConfigArgs
{
    /// Loads a configuration using these arguments
    pub fn load_config(&self) -> Result<Config,Box<dyn std::error::Error>>
    {
        match &self.path
        {
            None =>
            {
                let default = user_config_path()?;
                Config::load(default)
            },
            Some(path) => Config::load(path)
        }
    }
}


pub fn _project_config_path() -> Result<PathBuf,Box<dyn std::error::Error>>
{
    let current = env::var("PWD")?;
    let parts = vec![current.as_str(), config::DEFAULT_FILENAME];
    let path = parts.iter().collect::<PathBuf>();
    Ok(path)
}


pub fn user_config_path() -> Result<PathBuf,Box<dyn std::error::Error>>
{
    let home = env::var("HOME")?;
    let parts = vec![home.as_str(), ".bbcmd", config::DEFAULT_FILENAME];
    let path = parts.iter().collect::<PathBuf>();
    Ok(path)
}
