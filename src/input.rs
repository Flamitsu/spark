use uefi::proto::console::text::{Input, Key, ScanCode}; // Import input text methods (protocols) 
use uefi::{boot, Result, ResultExt, system}; // Import the boot, result and resultext modules from the uefi crate
use uefi::println; // Import the println macro from the uefi crate

// Read the key through stdin method in the system module (crate: UEFI)
// The function exists whenever the key ESC is pressed (I will change this in the future)
pub fn read_keyboard_events() -> Result {
    // system::with_stdin obtiene un &mut Input de forma segura
    system::with_stdin(|input: &mut Input| -> Result {
        loop {
            // wait_for_key_event() devuelve Option<Event>
            let ev = match input.wait_for_key_event() {
                Some(e) => e,
                None => continue, // If there is not event, continue the loop             
            };

            // wait_for_event requiere &[Event]
            let mut events = [ev];
            boot::wait_for_event(&mut events).discard_errdata()?;

            // Read the key pressed (It can return NONE, so I will handle it later.)
            match input.read_key()? {
                Some(Key::Printable(ch)) => {
                    println!("Printable key: '{}'", ch);
                }
                Some(Key::Special(ScanCode::ESCAPE)) => {
                    println!("ESC pressed, exiting"); // Exiting the program if the ESC key is pressed
                    break;
                }
                Some(Key::Special(code)) => {
                    println!("Special key: {:?}", code);
                }
                None => {
                    // If there is anything that the UTF-16 does not catch, for example F1 or F2
                    // key strokes, it can return NONE
                }
            }
        }

        Ok(()) // Return status code
    })
}
