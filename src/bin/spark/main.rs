// File where is going to reside the code of the general binary
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_default();
        return;
    }
    // Arguments
    match args[1].as_str() {
        "install" => install(),
        "remove"  => remove(),
        _ => {
            eprintln!("Unknown argument: {}", args[1]);
            exit(1);
        }
    }
}
// Default argument (non argument)
fn run_default() {
    println!("Detect kernels...");
}
// Install argument
fn install() {
    println!("Installing the EFI binary...");
}
// Remove argument
fn remove() {
    println!("Remove");
}
