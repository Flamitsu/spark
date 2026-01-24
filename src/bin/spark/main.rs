use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod utils;
mod cli;
mod commands;
mod boot;
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
        "install" => commands::install::install(skip_conf, efi_bin),
        "remove" => commands::remove::remove_installation(skip_conf, efi_bin), 
        "help" => cli::show_help(), // Shows the help about the program
        "clean" => commands::clean::clean_entries(),
        "update" => commands::update::update_entries(),
        _ => { // If the user says something not contemplated before:
            eprintln!("Unknown argument: {}", argument[1]); // Unknown argument with the argument
            cli::show_help(); // Show help argument
            exit(1); // Exit the process 
        }
    }
}
