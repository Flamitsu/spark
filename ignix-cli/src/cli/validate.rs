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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::path::PathBuf;
use crate::errors::IgnixError;
use crate::errors::{io, cmd};
use std::io::{Write, stdin, stdout};
pub fn is_valid_efi_bin_path(route: &str) -> Result<PathBuf, IgnixError>{
    let path = PathBuf::from(route);
    if !path.exists() || path.extension().is_none_or(|ext| ext != "efi") {
        Err(io::Error::NotFound(path.display().to_string()))?;
    }
    Ok(path)
}

pub fn is_valid_install_path(route: &str) -> Result<PathBuf, IgnixError>{
    let path = PathBuf::from(route);
    if path.exists(){
        return Ok(path);
    }
    Err(io::Error::NotFound(path.display().to_string()))?
}

pub fn ask_user_confirmation(context: &str) -> Result<bool, IgnixError>{
    
    println!("Remember to use capital letters as shown:");
    println!("Type 'YES' to {} or 'NO' to cancel.",context);
    
    stdout().flush().ok();

    let mut lector = String::new();
    stdin().read_line(&mut lector).ok();
    
    match lector.trim(){
        "YES" => Ok(true),
        "NO" => Err(cmd::Error::UserAborted)?,
        _ => {
            eprintln!("The program did not understoot the input '{}', assuming 'NO'.",lector);
            Err(cmd::Error::UserAborted)?
        }
    }
}
