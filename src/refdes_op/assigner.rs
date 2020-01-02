use crate::sch::design::Design;
use crate::refdes_op::refdes::Refdes;
use regex::Regex;
use std::collections::{HashSet, HashMap};


pub struct Assigner
{

}


impl Assigner
{
    pub fn create(design: &mut Design) -> Result<Assigner,()>
    {
        let mut components = design.components_mut();

        let mut attributes = components.iter_mut()
            .flat_map(|complex| complex.attributes.items.iter_mut())
            .filter(|attribute| attribute.attribute_name().is_some())
            .filter(|t| t.attribute_name().unwrap().eq(&String::from(r"refdes")))
            .inspect(|i| println!("{:?}", i))
            .collect::<Vec<_>>();

        let mut refdes = attributes.iter_mut()
            .map(|r| {let z = r.attribute_value().unwrap().parse::<Refdes>().unwrap(); (r, z)})
            .collect::<Vec<_>>();


        &refdes.iter().for_each(|c| println!("REFDES = {:?}", c.1.to_string()));


        let prefixes =
        {
            &refdes.iter()
                .map(|r| r.1.prefix.to_string())
                .collect::<HashSet<_>>()
        };

        &prefixes.iter().for_each(|c| println!("{:?}", c));


        let mut counts =
        {
            let just = &refdes.iter().map(|p| &p.1).collect::<Vec<&Refdes>>();

            prefixes.iter()
                .map(|prefix| (prefix, largest(&just, prefix)))
                .collect::<HashMap<_,_>>()

        };

        &counts.iter().for_each(|c| println!("{:?}", c));

        {
            let mut process = refdes.iter_mut().filter(|a| a.1.number.is_none()).collect::<Vec<_>>();

            for r1 in &mut process
            {
                let prefix = &r1.1.prefix;

                let number = counts[&prefix] + 1;
                counts.insert(&prefix, number);

                let next = format!("{}{}{}", r1.1.prefix, number, r1.1.suffix);

                r1.0.set_attribute_value(next.to_string());
            }

            counts.iter().for_each(|c| println!("{:?}", c));
        }

        Ok(Assigner {})
    }
}

fn largest(list: &Vec<&Refdes>, prefix: &str) -> i32
{
    list.iter()
        .filter(|refdes| prefix == refdes.prefix)
        .map(|refdes| refdes.number.unwrap_or(0))
        .fold(0, i32::max)
}

fn refdes_number(input: &str) -> Option<i32>
{
    lazy_static!
    {
        static ref REGEX: Regex = Regex::new(r"^(\D+)(\d+|\?)(.*)").unwrap();
    }

    let capture = REGEX.captures(input)?;

    let prefix = capture.get(1).unwrap().as_str();
    let number = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();

    Some(number)
}


fn refdes_prefix(input: &str) -> Option<&str>
{
    lazy_static!
    {
        static ref REGEX: Regex = Regex::new(r"^(\D+)(\d+|\?)(.*)").unwrap();
    }

    let capture = REGEX.captures(input)?;

    let prefix = capture.get(1).unwrap().as_str();
    let suffix = capture.get(3).unwrap().as_str();

    Some(prefix)
}
