use std::path::PathBuf;


pub trait Loadable
{
    fn load(path: &PathBuf) -> Result<Box<Self>,Box<std::error::Error>>;
}