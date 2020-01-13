use crate::scd::drawing::Drawing;
use std::path::{Path, PathBuf};
use std::env;
use crate::cfg::project::Project;


pub struct Config
{
    /// Project level settings
    project: Option<Project>
}


impl Config
{
    pub fn load() -> Result<Config,Box<dyn std::error::Error>>
    {
        Self::load_with_path(project_config_path()?)
    }


    pub fn load_with_path<T: AsRef<Path>>(path: T) -> Result<Config,Box<dyn std::error::Error>>
    {
        let project = Project::load(path);

        let config = Config
        {
            project: project.ok()
        };

        Ok(config)
    }


    pub fn load_drawing(&self, name: &str) -> Result<Drawing, Box<dyn std::error::Error>>
    {
        self.project.as_ref().unwrap().load_drawing(name)
    }
}


pub fn project_config_path() -> Result<PathBuf,Box<dyn std::error::Error>>
{
    match env::var("PWD")
    {
        Err(e) => Err(Box::new(e)),
        Ok(current) =>
            {
                let parts = vec![current.as_str(), "bbcmd.conf"];
                let path = parts.iter().collect::<PathBuf>();
                Ok(path)
            }
    }
}


pub fn _user_config_path() -> Result<PathBuf,Box<dyn std::error::Error>>
{
    match env::var("HOME")
    {
        Err(e) => Err(Box::new(e)),
        Ok(home) =>
        {
            let parts = vec![home.as_str(), ".bbcmd", "bbcmd.conf"];
            let path = parts.iter().collect::<PathBuf>();
            Ok(path)
        }
    }
}


#[cfg(test)]
mod test
{
    use std::env;
    use std::path::PathBuf;
    use super::*;


    #[test]
    fn test()
    {
        let path = vec!
        [
            env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
            "tests",
            "data",
            "bbcmd.conf"
        ].iter().collect::<PathBuf>();

        let _config = Config::load_with_path(&path);
    }
}