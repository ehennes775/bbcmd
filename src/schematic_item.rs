use std::io::Write;


pub trait SchematicItem
{
    fn write_to(&self, writer: &mut Box<dyn Write>);
}
