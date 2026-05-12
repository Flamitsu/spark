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
use std::{fs::read_to_string, io::Read};
use crate::{config::Routes, errors::IgnixError};
/* entropy_source needs to be mutable, because if you read something you are "modifying" it.
In theory it is just modifying the cursor position.*/
pub fn get_random<SOURCE: Read>(mut entropy_source: SOURCE, buffer: &mut [u8]) 
-> Result<(), IgnixError>{
    entropy_source.read_exact(buffer)?;
    Ok(())
}

#[allow(unused)]
pub struct SystemInfo{
    pub options: String,
    pub title: String,
    pub sort_key: String
}
#[allow(unused)]
impl SystemInfo{
    pub fn new() -> Result<SystemInfo, IgnixError>{
        let options: String = Self::filter_cmdline(&read_to_string(Routes::CMDLINE)?); 
        Ok(
            SystemInfo{
                options,
                title: String::new(),
                sort_key: String::new()
            }
        )
    }
    pub fn get_info(){

    }
    pub fn get_options(){

    }
    pub fn get_title(){

    }
    pub fn get_sort_key(){

    }
    pub fn filter_cmdline(cmdline_buffer: &str) -> String{
        const INITRD_ARGUMENT: &str = "initrd=";
        let cmdline = cmdline_buffer.split_whitespace();
        let mut filtered_cmdline = String::new();
        for arg in cmdline{
            if arg.starts_with(INITRD_ARGUMENT){
                break;
            } else {
                filtered_cmdline.push_str(arg);
            }
        }
        return filtered_cmdline;
    }
}
