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
use crate::IgnixError;
use crate::errors::cmd;
use crate::config::Flag;
use std::path::PathBuf;
use crate::cli::validate;
#[allow(unused)]
pub fn parse_prefixed_arg( arg: &str, efi: &mut Option<PathBuf>) 
    -> Result<(), IgnixError> {
    /*if let Some(path) = arg.strip_prefix(Flag::INSTALL_ROUTE) {
        *route = Some(validate::is_valid_install_path(path)?);
    } else */
    if let Some(path) = arg.strip_prefix(Flag::EFI_BIN_PATH) {
        *efi = Some(validate::is_valid_efi_bin_path(path)?);
    } else {
        Err(cmd::Error::InvalidArgument(arg.to_string()))?
    }
    Ok(())
}
