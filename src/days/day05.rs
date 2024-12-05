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

fn solve_a(lines: &[String]) -> u32 {
    let rules: HashMap<u32, HashSet<u32>> = lines
        .iter()
        .take_while(|line| line.contains('|'))
        .map(|line| {
            let mut it = line.split('|').map(|s| s.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .fold(HashMap::new(), |mut rules, (before, after)| {
            rules.entry(after).or_default().insert(before);
            rules
        });
    lines
        .iter()
        .skip_while(|line| line.contains('|') || line.is_empty())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let seq: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            let seqset: HashSet<u32> = seq.iter().copied().collect();
            if seq
                .iter()
                .scan((true, HashSet::new()), |(valid, before), next| {
                    *valid = rules
                        .get(&next)
                        .map(|rule| {
                            before.is_superset(&rule.intersection(&seqset).copied().collect())
                        })
                        .unwrap_or(true);
                    before.insert(*next);
                    Some(*valid)
                })
                .all(|valid| valid)
            {
                Some(seq[seq.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
