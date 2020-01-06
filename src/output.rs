use std::path::PathBuf;


const SGR_GREEN: &str = "\x1B[1;32m";
const SGR_RED: &str = "\x1B[1;31m";
const SGR_RESET: &str = "\x1B[0m";


pub fn print_file_op(op: &str, path: &PathBuf)
{
    print!("{} {}... ", op, path.to_str().unwrap_or("file"));
}


pub fn println_result(result: &std::io::Result<()>)
{
    match result
    {
        Err(e) => println!("{}Error:{} {}", SGR_RED, SGR_RESET, e),
        Ok(_) => println_success("Ok")
    }
}


pub fn println_success(message: &str)
{
    println!("{}{}{}", SGR_GREEN, message, SGR_RESET)
}