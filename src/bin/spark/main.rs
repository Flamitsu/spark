use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod utils;
mod cli;
mod commands;
mod boot;
mod nvram;
// This is the main code flow for the general spark binary
fn main() {
    boot::guid::detect_devices();
    // This catchs the user input
    let argument: Vec<String> = env::args().collect(); // Detect user input
    // This is when the argument is a dry-run (No argument) 
    if argument.len() < 2 {
        cli::show_help();
        return;
    } 
    // If the user uses '-y or --yes' as argument, it skips confirmation.
    let skip_conf = cli::skip_confirmation(&argument);
    // If the user uses --efi-bin={path} as argument, it uses it to install it in the ESP
    let efi_bin = cli::get_efi_bin_path(&argument);
    // Converts the collected argument into a string and divides it into different options.
    match argument[1].as_str() { 
        "install" => commands::install::install(skip_conf, efi_bin),
        "remove" => commands::remove::remove_installation(skip_conf, efi_bin), 
        "help" => cli::show_help(),
        "clean" => commands::clean::clean_entries(),
        "update" => commands::update::update_entries(),
        // If the user says something that the program can't understand, then:
        _ => {
            eprintln!("Unknown argument: {}", argument[1]);
            cli::show_help();
            exit(1);
        }
    }
}
