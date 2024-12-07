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

fn can_solve(lhs: i64, acc: i64, rhs: &[i64]) -> bool {
    match rhs.len() {
        0 => lhs == acc,
        _ => can_solve(lhs, acc + rhs[0], &rhs[1..]) || can_solve(lhs, acc * rhs[0], &rhs[1..]),
    }
}

fn solve_a(equations: &[(i64, Vec<i64>)]) -> i64 {
    equations
        .iter()
        .filter(|(lhs, rhs)| can_solve(*lhs, 0, rhs))
        .map(|(lhs, _)| lhs)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let equations: Vec<(i64, Vec<i64>)> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (lhs, rhs) = line.split_once(':').unwrap();
            (
                lhs.parse().unwrap(),
                rhs.trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    (solve_a(&equations).to_string(), "".to_string())
}