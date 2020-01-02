use regex::Regex;
use std::str::FromStr;
use std::fmt::{Debug, Formatter, Error};
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Refdes
{
    pub(crate) number: Option<i32>,

    pub prefix: String,

    pub suffix: String
}


const UNASSIGNED: &str = "?";


impl Debug for Refdes
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "Refdes {{ \"{}\" }}", self.to_string())
    }
}


impl FromStr for Refdes
{
    type Err = ();


    fn from_str(input: &str) -> Result<Self, Self::Err>
    {
        lazy_static!
        {
            static ref REGEX: Regex = Regex::new(r"^([[:alpha:]]+)(\d+|\?)(.*)").unwrap();
        }

        match REGEX.captures(input)
        {
            None =>
                {
                    Err(())
                }
            Some(capture) =>
                {
                    let number = match capture.get(2).unwrap().as_str()
                    {
                        UNASSIGNED => None,
                        n => match n.parse::<i32>()
                        {
                            Err(_e) => return Err(()),
                            Ok(0) => return Err(()),
                            Ok(t) => Some(t)
                        }
                    };

                    let prefix = capture.get(1).unwrap().as_str().to_string();

                    let suffix = capture.get(3).unwrap().as_str().to_string();

                    Ok(Refdes { number, prefix, suffix })
                }
        }
    }
}


impl PartialEq for Refdes
{
    fn eq(&self, other: &Self) -> bool
    {
        self.prefix.eq(&other.prefix) &&
        self.number.eq(&other.number) &&
        self.suffix.eq(&other.suffix)
    }
}


impl Ord for Refdes
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.partial_cmp(other).unwrap()
    }
}


impl PartialOrd for Refdes
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.prefix.partial_cmp(&other.prefix)
            .or(self.number.partial_cmp(&other.number))
            .or(self.suffix.partial_cmp(&other.suffix))
            .or(Some(Ordering::Equal))
    }
}


impl ToString for Refdes
{
    fn to_string(&self) -> String
    {
        match self.number
        {
            None =>
                {
                    format!("{}{}{}", self.prefix, UNASSIGNED, self.suffix)
                }
            Some(n) =>
                {
                    format!("{}{}{}", self.prefix, n, self.suffix)
                }
        }
    }
}


impl Refdes
{
    pub fn new(prefix: &str, number: i32, suffix: &str) -> Refdes
    {
        Refdes
        {
            prefix: String::from(prefix),
            number: Some(number),
            suffix: String::from(suffix)
        }
    }


    pub fn assign(&mut self, number: i32)
    {
        self.number = Some(number);
    }


    pub fn reset(&mut self)
    {
        self.number = None;
    }
}


#[cfg(test)]
mod test
{
    use crate::refdes_op::refdes::Refdes;


    #[test]
    fn test_assign()
    {
        let cases = vec![
            ("C1",  "C7" ),
            ("D2A", "D7A"),
            ("L3",  "L7" ),
            ("Q4A", "Q7A"),
            ("R?A", "R7A"),
            ("U?A", "U7A"),
        ];

        for case in cases
        {
            let mut refdes = case.0.parse::<Refdes>().unwrap();

            refdes.assign(7);

            assert_eq!(case.1, refdes.to_string())
        }
    }


    #[test]
    fn test_parse_and_back()
    {
        let cases = vec!
        [
            ( "C1",   Ok(("C",  Some(1), "" )) ),
            ( "D2A",  Ok(("D",  Some(2), "A")) ),
            ( "L3",   Ok(("L",  Some(3), "" )) ),
            ( "Q4A",  Ok(("Q",  Some(4), "A")) ),
            ( "R?A",  Ok(("R",  None,    "A")) ),
            ( "U?A",  Ok(("U",  None,    "A")) ),
            ( "not",  Err(())                  ),
            ( "123",  Err(())                  ),
            ( "Y0",   Err(())                  )
        ];

        for case in cases
        {
            match case.0.parse::<Refdes>()
            {
                Err(_e) =>
                    {
                        assert!(case.1.is_err())
                    },
                Ok(t) =>
                    {
                        assert!(case.1.is_ok());

                        assert_eq!(t.prefix, case.1.unwrap().0);
                        assert_eq!(t.number, case.1.unwrap().1);
                        assert_eq!(t.suffix, case.1.unwrap().2);

                        assert_eq!(case.0, t.to_string())
                    }
            }
        }
    }


    #[test]
    fn test_reset()
    {
        let cases = vec!
        [
            ( "C1",   "C?" ),
            ( "D2A",  "D?A"),
            ( "L3",   "L?" ),
            ( "Q4A",  "Q?A"),
            ( "R?A",  "R?A"),
            ( "U?A",  "U?A"),
        ];

        for case in cases
        {
            let mut refdes = case.0.parse::<Refdes>().unwrap();

            refdes.reset();

            assert_eq!(case.1, refdes.to_string())
        }
    }
}
