use crate::sch::design::Design;
use crate::sch::complex::Complex;
use crate::sch::text::Text;
use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::refdes_op::assigner::Assigner;
use crate::refdes_op::refdes::Refdes;
use crate::refdes_op::refdes_counters::RefdesCounters;
use std::io::stdout;
use std::fs::File;
use crate::output::{println_result, print_file_op};


#[derive(Debug, StructOpt)]
pub struct RefdesSubcommand
{
    #[structopt(long="assign", short="a")]
    /// Assign reference designators
    assign: bool,


    #[structopt(long="counts", short="c")]
    /// Reference designator counts input/output file
    counts : Option<PathBuf>,


    #[structopt(long="reset", short="r")]
    /// Reset reference designators
    reset : bool,


    #[structopt(parse(from_os_str), required=true)]
    /// Schematic input files
    files : Vec<PathBuf>
}


impl RefdesSubcommand
{
    pub fn execute(&self) -> Result<(),&str>
    {
        let mut schematics = Design::create(&self.files)?;

        let mut components = schematics.components_mut();

        let mut refdes_attributes = components.iter_mut()
            .flat_map(|complex| complex.attributes.items.iter_mut())
            .filter(|text| text.attribute_name().is_some())
            .filter(|text| text.attribute_name().unwrap().eq(&String::from(r"refdes")))
            .collect::<Vec<_>>();

        if self.reset
        {
            println!("{}", "Resetting REFDES...");

            for attribute in &mut refdes_attributes
            {
                let input = attribute.attribute_value().unwrap();

                match input.parse::<Refdes>()
                {
                    Err(()) =>
                        {
                            println!("invalid REFDES {}", input);
                        }
                    Ok(mut t) =>
                        {
                            t.reset();
                            attribute.set_attribute_value(t.to_string());
                        }
                }
            }
        }

        let mut counters = RefdesCounters::new();

        for attribute in &refdes_attributes
        {
            let input = attribute.attribute_value().unwrap();

            match input.parse::<Refdes>()
            {
                Err(()) =>
                    {
                        println!("invalid REFDES {}", input);
                    }
                Ok(t) =>
                    {
                        counters.push(&t);
                    }
            }
        }

        if self.assign
        {
            if let Some(t) = &self.counts
            {
                print_file_op("Reading REFDES counter file", t);

                let file = File::open(t).unwrap();
                let result = counters.read_from(file);

                println_result(result);
            }

            println!("{}", "Renumbering REFDES...");

            let assigner = Assigner::create(&mut schematics);
        }

        if let Some(t) = &self.counts
        {
            print_file_op("Writing REFDES counter file", t);

            let file = File::create(t).unwrap();
            let result = counters.write_to(file);

            println_result(result);
        }

        //schematics.dump();

        schematics.write();

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
    use crate::refdes_op::refdes_subcommand::assign_refdes;
    use crate::refdes_op::refdes_subcommand::reset_refdes;


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