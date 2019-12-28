#[macro_use] extern crate lazy_static;

mod check;
mod refdes;
mod arguments;
mod schematic_arc;
mod schematic_box;
mod schematic_bus;
mod schematic_circle;
mod schematic_complex;
mod schematic_page;
mod schematic_set;
mod schematic_line;
mod schematic_net;
mod schematic_pin;
mod schematic_path;
mod schematic_item;
mod schematic_text;
mod item_attributes;
mod item_params;

use std::process;
use structopt::StructOpt;

use crate::arguments::Arguments;


fn main()
{
    let arguments = Arguments::from_args();

    let result = arguments.execute();

    let status = match result
    {
        Err(e) =>
            {
                eprintln!("{}", e);
                -1
            },
        Ok(_t) => 0
    };

    process::exit(status);
}
