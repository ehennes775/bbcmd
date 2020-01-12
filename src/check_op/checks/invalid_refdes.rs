use crate::sch::design::Design;
use crate::check_op::error::Error;
use crate::check_op::check::Check;
use crate::refdes_op::refdes::Refdes;
use std::fmt::{Debug, Display};
use serde::export::Formatter;


pub struct InvalidRefdes
{
    page: String,
    refdes: String
}


impl Check for InvalidRefdes
{
    /// Checks the design for invalid reference designators
    fn check(design: &Design) -> Vec<Box<dyn Error>>
    {
        let mut errors: Vec<Box<dyn Error>> = vec![];

        for page in &design.pages
        {
            let attributes = page.components().iter()
                .flat_map(|a| a.attributes.items.iter())
                .filter(|a| a.attribute_name().map_or(false, |n| n.eq("refdes")))
                .collect::<Vec<_>>();

            for attribute in attributes
            {
                let value = attribute
                    .attribute_value()
                    .unwrap();

                if let Err(_) = value.parse::<Refdes>()
                {
                    errors.push(Box::new(InvalidRefdes { page: page.id(), refdes: value }))
                }
            }
        }

        errors
    }
}


impl Debug for InvalidRefdes
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "InvalidRefdes {{ page={:?} refdes={:?} }}", self.page, self.refdes)
    }
}


impl Display for InvalidRefdes
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "Invalid REFDES {} on page {}", self.refdes.to_string(), self.page)
    }
}


impl Error for InvalidRefdes
{
}
