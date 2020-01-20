use crate::cfg::project::Project;
use crate::scd::drawing::Drawing;
use std::path::Path;


pub struct Config
{
    /// Project level settings
    project: Option<Project>
}


pub const DEFAULT_FILENAME: &str = "bbcmd.conf";

impl Config
{
    pub fn load<T: AsRef<Path>>(path: T) -> Result<Config,Box<dyn std::error::Error>>
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
            DEFAULT_FILENAME
        ].iter().collect::<PathBuf>();

        let _config = Config::load(&path);
    }
}