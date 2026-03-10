use std::env;
use crate::error::SparkError;
mod commands;
mod error;
mod prelude;
fn main(){
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(e) = run(){
        eprintln!("ERROR: {}",e);
        std::process::exit(1);
    }
}
fn run() -> Result<(), SparkError> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2{
        commands::help::show_help();
        return Ok(());
    }
    match arguments[1].as_str() {
        "install" => commands::install::install_spark()?,
        "remove" => commands::remove::remove_spark_installation()?,
        _ => return Err(SparkError::InvalidArgument(arguments[1].to_string()))
    }
    Ok(())
}
