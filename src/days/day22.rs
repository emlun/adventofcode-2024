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
    let trigger_sets: HashMap<i64, HashMap<u32, i64>> = inits
        .iter()
        .copied()
        .map(|init| {
            let triggers: HashMap<u32, i64> =
                std::iter::successors(Some(init), |secret| Some(next(*secret)))
                    .take(2001)
                    .scan((0, 0, 0, 0, 0), |(p0, p1, p2, p3, p4), secret| {
                        let price = secret % 10;
                        let out = Some((price, [*p2 - *p1, *p3 - *p2, *p4 - *p3, price - *p4]));
                        (*p0, *p1, *p2, *p3, *p4) = (*p1, *p2, *p3, *p4, price);
                        out
                    })
                    .skip(4)
                    .fold(HashMap::new(), |mut triggers, (price, [d0, d1, d2, d3])| {
                        let trigger_key = (((d0 + 10) as u32) << 15)
                            | (((d1 + 10) as u32) << 10)
                            | (((d2 + 10) as u32) << 5)
                            | ((d3 + 10) as u32);
                        triggers.entry(trigger_key).or_insert(price);
                        triggers
                    });

            (init, triggers)
        })
        .collect();

    let all_all_triggers: HashSet<&u32> = trigger_sets.values().flat_map(|t| t.keys()).collect();

    let best_profit: i64 = all_all_triggers
        .into_iter()
        .map(|trigger| {
            trigger_sets
                .values()
                .flat_map(|triggers| triggers.get(trigger))
                .sum::<i64>()
        })
        .max()
        .unwrap();

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
