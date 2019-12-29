use std::io::Write;
use crate::sch::item_params::ItemParams;
use crate::sch::schematic_text::SchematicText;
use crate::sch::item_attributes::ItemAttributes;


pub trait SchematicItem
{
    fn attributes(&self) -> Option<&ItemAttributes> { None }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { None }


    fn params(&self) -> &ItemParams;


    fn write_to(&self, writer: &mut Box<dyn Write>);
}
