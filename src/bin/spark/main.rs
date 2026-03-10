use crate::error::SparkError;
mod commands;
mod error;
fn main(){
    commands::help::show_help();
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(e) = run(){
        eprintln!("ERROR: {}",e);
        std::process::exit(1);
    }
}
fn run() -> Result<(), SparkError> {
    let arg = "";
    if arg.is_empty(){
        return Err(SparkError::InvalidArgument(arg.to_string()));
    }
    Ok(())
}
