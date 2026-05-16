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
pub struct InstallOptions {
    pub force: bool,
    pub allow_virtual: bool,
    pub no_nvram: bool,
    pub removable_device: bool,
    pub efi_bin: PathBuf,
}

pub struct RemoveOptions {
    pub force: bool,
}

#[allow(unused)]
pub struct AddOptions{
    pub title: String,
    pub kernel_version: String,
    pub machine_id: String,
    pub sort_key: String,
    pub options: String,
    pub linux: String,
    pub initrd: Vec<String>
}
/*
 * This is because the bash hook needs to know the esp mountpoint and i refuse to
 * depend on findmnt etc. That is why i added a new flag named "hook"
*/
pub struct HookHelp{
    pub get_machine_id: bool,
    pub get_esp_mountpoint: bool,
}
