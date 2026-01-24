use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod kernel; // Module for auto_detect the path of the kernels already installed 
mod remove; // For removing the Spark installation in the system
mod install; // For installing the Spark program in the system
mod utils;
mod cli;
mod clean;
mod update;
mod esp;
// This is the main code flow for the general spark binary
fn main() {
    let argument: Vec<String> = env::args().collect(); // Detect user input
    if argument.len() < 2 { // dry-run
        cli::show_help(); // Shows help so the user know what to execute
        return;
    }
    // Arguments
    let skip_conf = cli::skip_confirmation(&argument); // Parses the argument  
    let efi_bin = cli::get_efi_bin_path(&argument); // Parses the argument 
    match argument[1].as_str() { // Converts the argument into string 
        "install" => install::install(skip_conf, efi_bin),
        "remove" => remove::remove_installation(skip_conf, efi_bin), 
        "help" => cli::show_help(), // Shows the help about the program
        "clean" => clean::clean_entries(),
        "update" => update::update_entries(),
        _ => { // If the user says something not contemplated before:
            eprintln!("Unknown argument: {}", argument[1]); // Unknown argument with the argument
            cli::show_help(); // Show help argument
            exit(1); // Exit the process 
        }
    }
}
