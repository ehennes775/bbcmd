use crate::sch::item_attributes::ItemAttributes;
use crate::sch::item_params::ItemParams;
use std::io::Write;
use std::fmt::Debug;
use crate::sch::text::Text;
use crate::sch::complex::Complex;


pub trait Item : Debug
{
    fn attributes(&self) -> Option<&ItemAttributes> { None }
    fn attributes_mut(&mut self) -> Option<&mut ItemAttributes> { None }


    fn params(&self) -> &ItemParams;


    fn into_complex(&self) -> Option<&Complex> { None }
    fn into_complex_mut(&mut self) -> Option<&mut Complex> { None }

    fn into_text(&self) -> Option<&Text> { None }


    fn write_to(&self, writer: &mut Box<dyn Write>) -> std::io::Result<()>;
}
