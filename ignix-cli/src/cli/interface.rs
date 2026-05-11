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
use crate::config::{Flag, Routes};
use crate::errors::IgnixError;
use crate::cli::{validate, parser};
use crate::cli::args::{InstallOptions, RemoveOptions};

pub fn parse_install_args(args: &[String]) -> Result<InstallOptions, IgnixError>{
    
    let mut force = false;
    let mut allow_virtual = false;
    let mut no_nvram = false;
    let mut removable_device = false;
    let mut install_route = None;
    let mut efi_bin_provided = None;

    for arg in args.iter().skip(2){
        
        match arg.as_str(){
            Flag::FORCE_FLAG => force = true,
            Flag::ALLOW_VIRTUAL_FLAG => allow_virtual = true,
            Flag::NO_NVRAM => no_nvram = true,
            Flag::REMOVABLE_FLAG => removable_device = true,
            _ => parser::parse_prefixed_arg(arg, &mut install_route, &mut efi_bin_provided)?
        }
    } 

    let efi_bin = match efi_bin_provided {
        Some(path) => path,
        None => validate::is_valid_efi_bin_path(Routes::DEFAULT_EFI_BIN_PATH)?,
    };

    Ok(InstallOptions {
        force,
        allow_virtual,
        no_nvram,
        removable_device,
        efi_bin,
        install_route
    })
}

pub fn parse_remove_args(args: &[String]) -> Result<RemoveOptions, IgnixError>{
    Ok(RemoveOptions {
        force: args.iter()
            .skip(2).any(|a| a == Flag::FORCE_FLAG)
    })
}
