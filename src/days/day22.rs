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

fn next(secret: i64) -> i64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn mix(secret: i64, mixin: i64) -> i64 {
    secret ^ mixin
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn solve_a(inits: &[i64]) -> i64 {
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

fn solve_b(inits: &[i64]) -> i64 {
    let mut solutions: HashMap<u32, i64> = HashMap::new();
    let mut best_profit = 0;

    for init in inits.iter().copied() {
        let triggers: HashMap<u32, i64> =
            std::iter::successors(Some(init), |secret| Some(next(*secret)))
                .take(2001)
                .scan((0, 0), |(trigger_key, p4), secret| {
                    let price = secret % 10;
                    *trigger_key = ((*trigger_key & 0x7fff) << 5) | ((price - *p4 + 10) as u32);
                    *p4 = price;
                    Some((price, *trigger_key))
                })
                .skip(4)
                .fold(HashMap::new(), |mut triggers, (price, trigger_key)| {
                    triggers.entry(trigger_key).or_insert(price);
                    triggers
                });

        for (trigger, price) in triggers {
            let sol = solutions.entry(trigger).or_default();
            let tot_profit = *sol + price;
            if tot_profit > best_profit {
                best_profit = tot_profit;
            }
            *sol = tot_profit;
        }
    }

    best_profit
}

pub fn solve(lines: &[String]) -> Solution {
    let inits: Vec<i64> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect();

    (solve_a(&inits).to_string(), solve_b(&inits).to_string())
}
