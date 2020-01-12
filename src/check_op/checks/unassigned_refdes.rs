use crate::sch::design::Design;
use crate::check_op::error::Error;
use crate::check_op::check::Check;
use crate::refdes_op::refdes::Refdes;
use std::fmt::{Debug, Display};
use serde::export::Formatter;


pub struct UnassignedRefdes
{
    page: String,
    refdes: Refdes
}


impl Check for UnassignedRefdes
{
    /// Checks the design for unassigned reference designators
    fn check(design: &Design) -> Vec<Box<dyn Error>>
    {
        let mut errors: Vec<Box<dyn Error>> = vec![];

        for page in &design.pages
        {
            for component in page.components()
            {
                let errors2 = component.attributes.items.iter()
                .filter(|a| a.attribute_name().map_or(false, |n| n.eq("refdes")))
                .filter_map(|a| a.attribute_value())
                .map(|v| v.parse::<Refdes>())
                .filter(|r| r.is_ok())
                .map(|v| v.unwrap())
                .filter(|r| r.number.is_none())
                .map(|refdes| UnassignedRefdes { page: page.id(), refdes })
                .collect::<Vec<_>>();

                for e in errors2
                {
                    errors.push(Box::new(e))
                }
            }
        }

        errors
    }
}


impl Debug for UnassignedRefdes
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "UnassignedRefdes {{ page={:?} refdes={:?} }}", self.page, self.refdes)
    }
}


impl Display for UnassignedRefdes
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "Unassigned REFDES {} on page {}", self.refdes.to_string(), self.page)
    }
}


impl Error for UnassignedRefdes
{
}
