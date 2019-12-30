use std::path::PathBuf;
use structopt::StructOpt;
use crate::sch::design::Design;
use crate::sch::complex::Complex;
use crate::sch::text::Text;
use regex::Regex;


#[derive(Debug, StructOpt)]
pub struct RefdesSubcommand
{
    #[structopt(long="renumber", short="n")]
    /// Renumber reference designators
    renumber :  bool,


    #[structopt(long="reset", short="r")]
    /// Reset reference designators
    reset : bool,


    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl RefdesSubcommand
{
    pub fn execute(&self) -> Result<(),(&str)>
    {
        let schematics = match Design::create(&self.files)
        {
            Err(e) => return Err(e),
            Ok(t) => t
        };

        println!("{:?}", schematics);

        for schematic in &schematics.pages
        {
            println!("    {:?}", schematic);

            for item in &schematic.items
            {
                println!("        {:?}", item);
            }
        }

        println!();
        println!("{}", "Components");

        let components: Vec<&Complex> = schematics.pages
            .iter()
            .map(|p| p.items.iter())
            .flatten()
            .map(|i| i.into_complex())
            .flat_map(|i| i)
            .collect();

        for component in components
        {
            println!("{:?}", component);

            let attributes = component.attributes.items
                .iter()
                .filter(|t| t.attribute_name().is_some())
                .filter(|t| t.attribute_name().unwrap().eq(&String::from(r"refdes")))
                .map(|t| t.attribute_value().unwrap());

            for attribute in attributes
            {
                println!("    {:?}", attribute);
            }
        }

        Ok(())
    }
}


fn assign_refdes<T: ToString>(input: &str, value: T) -> Option<String>
{
    lazy_static!
    {
        static ref REGEX: Regex = Regex::new(r"^(\D+)(\d+|\?)(.*)").unwrap();
    }

    let capture = REGEX.captures(input)?;

    let prefix = capture.get(1).unwrap().as_str();
    let suffix = capture.get(3).unwrap().as_str();

    Some(format!("{}{}{}", prefix, value.to_string(), suffix))
}


fn reset_refdes(input: &str) -> Option<String>
{
    assign_refdes(input, "?")
}


#[cfg(test)]
mod test
{
    use crate::refdes::refdes_subcommand::assign_refdes;
    use crate::refdes::refdes_subcommand::reset_refdes;


    #[test]
    fn test_assign_refdes()
    {
        let cases = vec!
        [
            ("C1",  10, Some("C10")),
            ("D1A", 15, Some("D15A")),
            ("L?",  20, Some("L20")),
            ("R?A", 25, Some("R25A")),
            ("NOT",  0, None),
            ("123",  0, None)
        ];

        for case in cases
        {
            let output = assign_refdes(case.0, case.1);

            assert_eq!(output, case.2.and_then(|s| Some(s.to_string())));
        }
    }


    #[test]
    fn test_reset_refdes()
    {
        let cases = vec!
        [
            ("C1",  Some("C?")),
            ("D1A", Some("D?A")),
            ("L?",  Some("L?")),
            ("R?A", Some("R?A")),
            ("NOT", None),
            ("123", None)
        ];

        for case in cases
        {
            let output = reset_refdes(case.0);

            assert_eq!(output, case.1.and_then(|s| Some(s.to_string())));
        }
    }
}