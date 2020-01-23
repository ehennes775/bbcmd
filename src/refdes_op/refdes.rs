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
        (&self.prefix, self.number, &self.suffix).cmp(
            &(&other.prefix, other.number, &other.suffix)
            )
    }
}


impl PartialOrd for Refdes
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
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
    fn test_eq()
    {
        let r1 = "R1".parse::<Refdes>();
        let r2 = "R1".parse::<Refdes>();

        assert_eq!(r1, r2);
    }


    #[test]
    fn test_ne_relational()
    {
        let c1 = "C1".parse::<Refdes>();
        let r1 = "R1".parse::<Refdes>();

        assert_ne!(c1, r1);
        assert!(c1 < r1);
        assert!(r1 > c1);

        let l1 = "L1".parse::<Refdes>();
        let l2 = "L2".parse::<Refdes>();

        assert_ne!(l1, l2);
        assert!(l1 < l2);
        assert!(l2 > l1);

        let d1a = "D1A".parse::<Refdes>();
        let d1b = "D1B".parse::<Refdes>();

        assert_ne!(d1a, d1b);
        assert!(d1a < d1b);
        assert!(d1b > d1a);
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
