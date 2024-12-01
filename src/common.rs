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

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub type Solution = (String, String);

pub fn day_input_filename(day: u8) -> PathBuf {
    let padded_day = format!("{day:02}");
    Path::new("inputs").join(format!("day{padded_day}.in"))
}

pub fn get_file_lines(path: &Path) -> Result<Vec<String>, std::io::Error> {
    if path == Path::new("-") {
        read_lines(std::io::stdin())
    } else {
        read_lines(File::open(path).unwrap_or_else(|_| panic!("Input file not found: {path:?}")))
    }
}

fn read_lines<I: Read>(mut source: I) -> Result<Vec<String>, std::io::Error> {
    let mut contents: String = String::new();
    source.read_to_string(&mut contents)?;
    Ok(contents.lines().map(&str::to_string).collect())
}
