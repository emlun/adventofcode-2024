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

use crate::{common::Solution, util::iter::WithPairs};

fn solve_a(map: &HashMap<char, Vec<(isize, isize)>>, maxr: isize, maxc: isize) -> usize {
    map.values()
        .flat_map(|antennae| {
            antennae.pairs().flat_map(|((ra, ca), (rb, cb))| {
                let dr = rb - ra;
                let dc = cb - ca;
                [(ra - dr, ca - dc), (rb + dr, cb + dc)]
            })
        })
        .filter(|(r, c)| (0..maxr).contains(r) && (0..maxc).contains(c))
        .collect::<HashSet<_>>()
        .len()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: HashMap<char, Vec<(isize, isize)>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, freq)| *freq != '.')
                .map(move |(c, freq)| (r as isize, c as isize, freq))
        })
        .fold(HashMap::new(), |mut map, (r, c, freq)| {
            map.entry(freq).or_default().push((r, c));
            map
        });

    let maxr = lines.iter().filter(|line| !line.is_empty()).count() as isize;
    let maxc = lines[0].len() as isize;

    (solve_a(&map, maxr, maxc).to_string(), "".to_string())
}
