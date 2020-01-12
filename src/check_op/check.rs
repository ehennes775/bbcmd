use crate::sch::design::Design;
use crate::check_op::error::Error;


pub(crate) trait Check
{
    fn check(design: &Design) -> Vec<Box<dyn Error>>;
}