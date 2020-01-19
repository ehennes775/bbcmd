use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::env;


#[derive(Deserialize)]
pub struct Project
{
    #[serde(skip_deserializing)]
    /// Maintains the absolute path of the project file for relative paths
    path: PathBuf,


    /// Paths for the files representing schematic pages
    schematics: Vec<PathBuf>
}


/// The default filename for a project, when the project file is not specified
const DEFAULT_PATH: &str = "bbcmd.bbproj";


impl Project
{
    /// Load the project from a file
    pub fn load<T: AsRef<Path>>(path: Option<T>) -> Result<Box<Project>,std::io::Error>
    {
        let mut current = env::current_dir()?;

        let path = match path
        {
            None =>
                {
                    current.push(DEFAULT_PATH);
                    current
                },
            Some(t) =>
                {
                    current.push(t);
                    current
                }
        };

        let file = File::open(&path)?;

        let reader = BufReader::new(file);

        let mut project: Project = serde_json::from_reader(reader)?;

        project.path = path;

        Ok(Box::new(project))
    }


    /// Paths for the files representing schematic pages
    pub fn schematics(&self) -> std::vec::IntoIter<PathBuf>
    {
        self.schematics.iter()
            .map(|p|
            {
                let mut absolute = PathBuf::from(self.path.parent().unwrap());
                absolute.push(p);
                absolute
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}


#[cfg(test)]
mod test
{
    use std::env;
    use std::path::PathBuf;
    use super::*;


    #[test]
    fn test_load()
    {
        let path = vec!
        [
            env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
            "tests",
            "data",
            DEFAULT_PATH
        ].iter().collect::<PathBuf>();

        let _project = Project::load(Some(&path)).unwrap();

        assert_eq!(&path, &_project.path);
    }
}