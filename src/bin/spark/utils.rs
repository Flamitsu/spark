use std::io::{stdin, Write, stdout}; // Import the input output standard library 
// This is the archive where is going to be stored common code between the modules
pub fn confirmation(context: &str) -> bool{ // This function needs an string and returns a bool
    println!("Type 'YES' to {} spark, or 'NO' to cancel: ", context); // Prints the user what they 
    // need to input
    stdout().flush().unwrap();
    let mut decision = String::new(); // Creates a new string 
    stdin().read_line(&mut decision).unwrap(); // Read the input 
    match decision.trim() { // Match the options
        "YES" => return true, // If the user said yes all caps, it returns true 
        "NO" => return false, // If the user said no all caps, it returns false
        _ => {
            println!("The program did not understood the input. Assuming 'NO'."); // If the user
            // said something that is not 'NO' or 'YES' it returns false
            return false
        }
    }
}
