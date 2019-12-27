use std::io::Write;
use crate::item_params::ItemParams;


pub trait SchematicItem
{
    fn params(&self) -> &ItemParams;


    fn write_to(&self, writer: &mut Box<dyn Write>);
}
