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

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::common::Solution;

fn solve_a(connections: &HashMap<&str, HashSet<&str>>) -> usize {
    let groups: BTreeSet<BTreeSet<&str>> = connections
        .iter()
        .flat_map(|(a, to)| {
            to.iter().flat_map(|b| {
                to.iter()
                    .filter(move |c| *c != b)
                    .filter(|c| connections[b].contains(**c))
                    .copied()
                    .map(|c| [a, b, c].into_iter().collect())
            })
        })
        .collect();
    groups
        .into_iter()
        .filter(|computers| computers.len() == 3 && computers.iter().any(|c| c.starts_with('t')))
        .count()
}

fn solve_b(connections: &HashMap<&str, HashSet<&str>>) -> String {
    let groups: Vec<BTreeSet<&str>> =
        connections.keys().fold(Vec::new(), |mut groups, computer| {
            let mut new_groups = Vec::new();
            for group in groups.iter_mut() {
                if group.iter().all(|a| connections[a].contains(computer)) {
                    new_groups.push(group.clone());
                    group.insert(computer);
                }
            }
            groups.push([*computer].into_iter().collect());
            groups.extend(new_groups);
            groups
        });

    let mut names: Vec<&str> = groups
        .into_iter()
        .max_by_key(|computers| computers.len())
        .unwrap()
        .into_iter()
        .collect();
    names.sort();
    names.join(",")
}

pub fn solve(lines: &[String]) -> Solution {
    let connections: HashMap<&str, HashSet<&str>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .fold(HashMap::new(), |mut connections, line| {
            let (a, b) = line.trim().split_once('-').unwrap();
            connections.entry(a).or_default().insert(b);
            connections.entry(b).or_default().insert(a);
            connections
        });

    (
        solve_a(&connections).to_string(),
        solve_b(&connections).to_string(),
    )
}
