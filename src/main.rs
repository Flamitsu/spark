#![no_std]
#![no_main]
use log::info;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, Char16, Result, ResultExt};
use uefi::prelude::*;
#[entry]
fn main() {
    let mut input: &mut Input;
    loop {
        // Pause until a keyboard event occurs.
        let mut events = [input.wait_for_key_event().unwrap()];
        boot::wait_for_event(&mut events).discard_errdata();

        let u_key = Char16::try_from('u').unwrap();
        match input.read_key()? {
            // Example of handling a printable key: print a message when
            // the 'u' key is pressed.
            Some(Key::Printable(key)) if key == u_key => {
                info!("the 'u' key was pressed");
            }

            // Example of handling a special key: exit the loop when the
            // escape key is pressed.
            Some(Key::Special(ScanCode::ESCAPE)) => {
                break;
            }
            _ => {}
        }
    }
    Ok(());
}
