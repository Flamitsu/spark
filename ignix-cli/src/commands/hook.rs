use crate::boot::disk::DiskScanner;
use crate::cli::args::HookHelp;

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
use crate::IgnixError;
use crate::errors::cmd;
use crate::utils::SystemInfo;
pub fn help_hooks(options: HookHelp) -> Result<String, IgnixError>{
    if options.get_machine_id && !options.get_esp_mountpoint{
        return Ok(SystemInfo::new()?.machine_id.to_string());
    }

    if options.get_esp_mountpoint && !options.get_machine_id {
        return Ok(DiskScanner::new(false, false).find_compatible_esp()?
            .mountpoint.display().to_string())
    }
    Err(cmd::Error::InvalidArgument("invalid hook help arguments".into()))?
}
