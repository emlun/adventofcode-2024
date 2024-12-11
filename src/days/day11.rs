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

use crate::{common::Solution, util::iter::Countable};

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

fn simulate(mut stones: HashMap<u64, usize>, steps: usize) -> HashMap<u64, usize> {
    for _ in 0..steps {
        stones = stones
            .into_iter()
            .fold(HashMap::new(), |mut new_stones, (value, count)| {
                let (s, t) = step(value);
                *new_stones.entry(s).or_insert(0) += count;
                if let Some(t) = t {
                    *new_stones.entry(t).or_insert(0) += count;
                }
                new_stones
            });
    }
    stones
}

pub fn solve(lines: &[String]) -> Solution {
    let stones: HashMap<u64, usize> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_whitespace())
        .map(|ch| ch.parse().unwrap())
        .counts();

    let stones = simulate(stones, 25);
    let solution_a: usize = stones.values().sum();

    let solution_b: usize = simulate(stones, 50).values().sum();
    (solution_a.to_string(), solution_b.to_string())
}
