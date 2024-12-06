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

use std::collections::{HashMap, HashSet};

use crate::common::Solution;

fn solve_a(map: &[Vec<bool>], start: (usize, usize, u8)) -> usize {
    std::iter::successors(Some(start), |(r, c, dir)| {
        let (rr, cc) = match dir {
            0 => (r.checked_sub(1)?, *c),
            1 => (*r, c + 1),
            2 => (r + 1, *c),
            3 => (*r, c.checked_sub(1)?),
            _ => unreachable!(),
        };
        Some(if *map.get(rr)?.get(cc)? {
            (*r, *c, (dir + 1) % 4)
        } else {
            (rr, cc, *dir)
        })
    })
    .map(|(r, c, _)| (r, c))
    .collect::<HashSet<_>>()
    .len()
}

pub fn solve(lines: &[String]) -> Solution {
    let (start, map): (Option<(usize, usize, u8)>, Vec<Vec<bool>>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (None, Vec::with_capacity(lines.len())),
            |(mut start, mut map), (r, line)| {
                map.push(
                    line.chars()
                        .enumerate()
                        .map(|(c, ch)| match ch {
                            '#' => true,
                            '^' => {
                                start = Some((r, c, 0));
                                false
                            }
                            _ => false,
                        })
                        .collect(),
                );
                (start, map)
            },
        );
    (
        solve_a(&map, start.expect("Failed to find start position")).to_string(),
        "".to_string(),
    )
}
