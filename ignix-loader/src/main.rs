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
#![no_std]
#![no_main]
#![allow(unused)]
mod panic_handler;
mod uefi;
use uefi::Status;

#[unsafe(no_mangle)]
// This is because extern "C" follows the System V, not the ms_abi (needed for an UEFI app)
extern "efiapi" fn efi_main() -> Status { 
    Status::SUCCESS
}
