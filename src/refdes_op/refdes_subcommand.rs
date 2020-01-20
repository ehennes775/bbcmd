use crate::sch::design::Design;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::refdes_op::refdes::Refdes;
use crate::refdes_op::refdes_counters::RefdesCounters;
use crate::cfg::config_args::ConfigArgs;


#[derive(Debug, StructOpt)]
pub struct RefdesSubcommand
{
    #[structopt(long="assign", short="a")]
    /// Assign reference designators
    assign: bool,


    #[structopt(flatten)]
    config_args: ConfigArgs,


    #[structopt(long="counts", short="n")]
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
    pub fn execute(&self) -> Result<(),Box<dyn std::error::Error>>
    {
        let _config = self.config_args.load_config()?;
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
                counters.read_from_file(t).unwrap();
            }

            println!("{}", "Renumbering REFDES...");

            for attribute in refdes_attributes
            {
                let mut refdes = attribute.attribute_value().unwrap().parse::<Refdes>().unwrap();

                if refdes.number.is_none()
                {
                    counters.assign_number(&mut refdes);

                    attribute.set_attribute_value(refdes.to_string());
                }
            }
        }

        if let Some(t) = &self.counts
        {
            counters.write_to_file(t).unwrap();
        }

        //schematics.dump();

        schematics.write();

        Ok(())
    }
}


#[cfg(test)]
mod test
{
}