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

use std::collections::HashMap;

use crate::common::Solution;

fn count_solutions<'pat>(
    patterns: &'pat [&'pat str],
    rest: &'pat str,
    memo: &mut HashMap<&'pat str, usize>,
) -> usize {
    memo.get(rest).copied().unwrap_or_else(|| {
        let solutions = patterns
            .iter()
            .flat_map(|pat| rest.strip_prefix(pat))
            .map(|rest| count_solutions(patterns, rest, memo))
            .sum();
        memo.insert(rest, solutions);
        solutions
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let patterns: Vec<&str> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.split(','))
        .map(|s| s.trim())
        .collect();

    let mut memo = HashMap::new();
    memo.insert("", 1);
    let (solution_a, solution_b): (usize, usize) = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .fold((0, 0), |(a, b), goal| {
            let sol = count_solutions(&patterns, goal, &mut memo);
            (a + if sol > 0 { 1 } else { 0 }, b + sol)
        });

    (solution_a.to_string(), solution_b.to_string())
}
