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

fn next(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn mix(secret: u64, mixin: u64) -> u64 {
    secret ^ mixin
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn solve_a(inits: &[u64]) -> u64 {
    inits
        .iter()
        .copied()
        .map(|init| {
            std::iter::successors(Some(init), |secret| Some(next(*secret)))
                .nth(2000)
                .unwrap()
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let inits: Vec<u64> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (solve_a(&inits).to_string(), "".to_string())
}
