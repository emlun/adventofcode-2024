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

use crate::common::Solution;

fn solve_a(lines: &[String]) -> usize {
    (0..lines.len())
        .flat_map(|r| (0..lines[r].len()).map(move |c| (r as isize, c as isize)))
        .map(|(r, c)| {
            (-1_isize..=1)
                .flat_map(|dr| (-1_isize..=1).map(move |dc| (dr, dc)))
                .filter(|drc| *drc != (0, 0))
                .filter(|(dr, dc)| {
                    (0..lines.len() as isize).contains(&(r + 3 * dr))
                        && (0..lines[0].len() as isize).contains(&(c + 3 * dc))
                })
                .filter(|(dr, dc)| {
                    lines[r as usize].chars().nth(c as usize) == Some('X')
                        && lines[(r + dr) as usize].chars().nth((c + dc) as usize) == Some('M')
                        && lines[(r + 2 * dr) as usize]
                            .chars()
                            .nth((c + 2 * dc) as usize)
                            == Some('A')
                        && lines[(r + 3 * dr) as usize]
                            .chars()
                            .nth((c + 3 * dc) as usize)
                            == Some('S')
                })
                .count()
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
