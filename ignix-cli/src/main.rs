/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
mod errors;
pub mod config; // Here is where all the consts resides centralized
pub mod boot;
mod cli;
mod commands;
use std::env;
mod utils;
use crate::errors::IgnixError;
use crate::errors::cmd;
fn main(){
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(error) = run(){
        eprintln!("{}",error);
        std::process::exit(1);
    }
}

/// The run function is the one that runs the program. If there is some problem it will tell it about it to the main function and the main function will exit the program with a message.
fn run() -> Result<(), IgnixError> {
    let args: Vec<String> = env::args().collect();
    
    // If there is not any argument, it will show the help.
    if args.len() < 2{
        commands::help::show_help();
        return Ok(());
    }
    
    // Converts the second argument into string and starts matching
    match args[1].as_str() {
        "add" => {
            let options = cli::interface::parse_add_args(&args)?;
            commands::add::add_entry(options)?;
        }
        "install" => {
            let options = cli::interface::parse_install_args(&args)?;
            commands::install::install_ignix(options)?;
        },
        "uninstall" => {
            let options = cli::interface::parse_remove_args(&args)?;
            commands::uninstall::remove_ignix(options)?;
        },
        "help" => commands::help::show_help(),
        _ => return Err(cmd::Error::InvalidArgument(args[1].to_string()))?
    }
    Ok(())
}
