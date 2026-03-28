mod boot;
mod cli;
mod commands;
mod errors;
mod config; // Here is where all the consts resides centralized
use std::env;
use crate::errors::IgnixError;
use crate::errors::cmd;

fn main(){
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(error) = run(){
        eprintln!("{}",error);
        std::process::exit(1);
    }
}

/// The run function is the one that runs the program. If there is some problem it will tell it to the main function and the main function will exit the program with a message.
fn run() -> Result<(), IgnixError> {
    let args: Vec<String> = env::args().collect();
    
    // If there is not any argument, it will show the help.
    if args.len() < 2{
        commands::help::show_help();
        return Ok(());
    }
    
    // Converts the second argument into string and starts matching
    match args[1].as_str() {
        "install" => {
            let options = cli::parse_install_args(&args)?;
            commands::install::install_ignix(options)?
        },
        "remove" => {
            let options = cli::parse_remove_args(&args)?;
            commands::remove::remove_ignix_installation(options)?
        },
        "update" => commands::update::update_entries()?,
        "check" => todo!("This command should check the configs and the kernels"),
        "list" => todo!("This command should list all the aviable kernels configured"),
        "clean" => commands::clean::clean_entries()?,
        "help" => commands::help::show_help(),
        _ => return Err(cmd::Error::InvalidArgument(args[1].to_string()))?
    }
    Ok(())
}
