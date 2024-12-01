// Solutions to Advent of Code 2024
// Copyright (C) 2024  Emil Lundberg <emil@emlun.se>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub fn crate_author() -> &'static str {
    option_env!("CARGO_PKG_AUTHORS")
        .and_then(|authors| authors.split(',').next())
        .unwrap_or("<unknown>")
}

pub fn crate_description() -> &'static str {
    option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("")
}

pub fn crate_name() -> &'static str {
    option_env!("CARGO_PKG_NAME").unwrap_or("<program name not set>")
}

pub fn crate_version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
}
