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

fn step(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else {
        let log = stone.ilog10() + 1;
        if log % 2 == 0 {
            let divisor = 10_u64.pow(log / 2);
            (stone / divisor, Some(stone % divisor))
        } else {
            (stone * 2024, None)
        }
    }
}

fn solve_a(mut stones: Vec<u64>) -> usize {
    for _ in 0..25 {
        for i in 0..stones.len() {
            match step(stones[i]) {
                (s, None) => {
                    stones[i] = s;
                }
                (s, Some(t)) => {
                    stones[i] = s;
                    stones.push(t);
                }
            }
        }
    }
    stones.len()
}

pub fn solve(lines: &[String]) -> Solution {
    let stones = lines
        .iter()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_whitespace())
        .map(|ch| ch.parse().unwrap())
        .collect();

    (solve_a(stones).to_string(), "".to_string())
}
