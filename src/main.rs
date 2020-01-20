#[macro_use] extern crate lazy_static;

mod arguments;
mod check_op;
mod refdes_op;
mod ebom_op;
mod sch;
mod library;
mod output;
mod cfg;
mod prj;

mod scd;

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
