// This is the main code flow for the general spark binary.
use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod auto_detect; // Module for auto_detect the path of the kernels already installed 
mod remove; // For removing the Spark installation in the system
mod install; // For installing the Spark program in the system
mod utils; // De-duplicated code library
fn main() {
    let argument: Vec<String> = env::args().collect();
    if argument.len() < 1 {
        auto_detect::detect_new_kernel();
        return;
    }
    // Arguments
    match argument[1].as_str() {
        "install" => install::install(),
        "remove"  => remove::remove_installation(),
        _ => {
            eprintln!("Unknown argument: {}", argument[1]);
            exit(1);
        }
    }
}
