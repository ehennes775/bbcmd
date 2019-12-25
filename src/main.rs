#[macro_use]
extern crate lazy_static;

mod check;
mod refdes;
mod arguments;

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
