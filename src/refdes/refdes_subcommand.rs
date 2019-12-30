use std::path::PathBuf;
use structopt::StructOpt;
use crate::sch::design::Design;
use crate::sch::complex::Complex;
use crate::sch::text::Text;
use regex::Regex;
use std::io::{stdout, Write};
use std::ffi::OsString;
use std::fs::File;


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
        let mut schematics = match Design::create(&self.files)
        {
            Err(e) => return Err(e),
            Ok(t) => t
        };

        println!();
        println!("{}", "Components");

        {
            let mut components: Vec<&mut Complex> = schematics.pages
            .iter_mut()
            .map(|p| p.items.iter_mut())
            .flatten()
            .map(|i| i.into_complex_mut())
            .flat_map(|i| i)
            .collect();

            for component in components
            {
                //println!("{:?}", component);

                let mut attributes: Vec<&mut Text> = component.attributes.items
                .iter_mut()
                .filter(|t| t.attribute_name().is_some())
                .filter(|t| t.attribute_name().unwrap().eq(&String::from(r"refdes")))
                //.flat_map(|i| i)
                .collect()
                ;

                for attribute in attributes
                {
                    let input = attribute.attribute_value().unwrap();

                    match reset_refdes(input.as_str())
                    {
                        None =>
                            {
                                println!("invalid REFDES {}", input);
                            }
                        Some(t) =>
                            {
                                println!("valid before={:?} after={:?}", input, t);

                                attribute.set_attribute_value(t);
                            }
                    }
                }
            }
        }

        // TODO: move into a debug/dump function

        println!("{:?}", schematics);

        for schematic in &schematics.pages
        {
            println!("    {:?}", schematic);

            for item in &schematic.items
            {
                println!("        {:?}", item);

                if let Some(c) = item.into_complex()
                {
                    for attribute in &c.attributes.items
                    {
                        println!("            {:?}", attribute);
                    }
                }
            }
        }


        {
            for page in schematics.pages
            {
                let output_filename = format!("out_{}", page.path.file_name().unwrap().to_str().unwrap());

                let x2 = OsString::from(&output_filename);

                let file = File::create(&output_filename).unwrap();

                println!("Writing {:?}", output_filename);

                let mut output: Box<dyn Write> = Box::new(file);

                page.write_to(&mut output);
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