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
#[derive(Debug)]
/// Errors related to the bad usage of the ignix command. Like for example an invalid argument 
#[allow(unused)]
pub enum Error {
    InvalidArgument(String),
    UserAborted,
    NotEFIPartitionFound,
    KeyValueMissing(String, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            Error::UserAborted => write!(f, "User aborted the process."),
            Error::NotEFIPartitionFound => write!(f, "Not UEFI partition found in the system."),
            Error::KeyValueMissing(arg, file) => write!(f, "Missing {} value in {}", arg,file), 
        }
    }
}

impl std::error::Error for Error {}
