use std::fmt::{Debug, Error, Formatter};
use crate::sch::complex::Complex;


#[derive(Eq, Hash, PartialEq)]
pub struct Key
{
    /// The filename, including extension, of the specification control drawing (SCD)
    scd: String,

    pub(crate) value: Option<String>
}


impl Debug for Key
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Part {{ s=\"{:?}\" v=\"{:?}\" }}", self.scd, self.value)
    }

}


//impl Eq for Key
//{
//
//}
//
//
//impl Hash for Key
//{
//    fn hash<H: Hasher>(&self, state: &mut H)
//    {
//        self.scd.hash(state);
//        self.value.hash(state);
//    }
//}


impl Key
{
    pub fn scd(&self) -> &str { &self.scd }


    pub fn create(component: &Complex) -> Option<Key>
    {
        let attributes = component.attributes.items.iter()
            .filter(|a| a.attribute_name().is_some())
            .collect::<Vec<_>>();

        let scd = attributes.iter()
            .filter(|a| a.attribute_name().unwrap().eq("scd"))
            .take(1)
            .collect::<Vec<_>>()
            .first()
            .and_then(|x| x.attribute_value())?;

        let value = attributes.iter()
            .filter(|a| a.attribute_name().unwrap().eq("value"))
            .take(1)
            .collect::<Vec<_>>()
            .first().and_then(|x| x.attribute_value());

        Some ( Key { scd, value } )
    }
}