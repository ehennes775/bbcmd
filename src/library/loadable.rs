use std::path::Path;


pub trait Loadable
{
    fn load<T: AsRef<Path>>(path: T) -> Result<Box<Self>,Box<dyn std::error::Error>>;
}