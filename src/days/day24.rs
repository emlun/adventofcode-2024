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

struct Gate<'gate> {
    a: &'gate str,
    b: &'gate str,
    op: Op,
}

enum Op {
    And,
    Or,
    Xor,
}

fn solve_a<'gate>(mut state: HashMap<&'gate str, bool>, gates: &HashMap<&'gate str, Gate>) -> u64 {
    while !gates
        .keys()
        .filter(|k| k.starts_with('z'))
        .all(|k| state.contains_key(k))
    {
        for (rhs, lhs) in gates.iter() {
            if !state.contains_key(*rhs) {
                if let (Some(&a), Some(&b)) = (state.get(lhs.a), state.get(lhs.b)) {
                    state.insert(
                        rhs,
                        match lhs.op {
                            Op::And => a && b,
                            Op::Or => a || b,
                            Op::Xor => (a && !b) || (!a && b),
                        },
                    );
                }
            }
        }
    }

    state
        .into_iter()
        .flat_map(|(k, v)| {
            let bit_i: u64 = k.strip_prefix('z')?.parse().ok()?;
            Some((bit_i, if v { 1 } else { 0 }))
        })
        .fold(0, |result, (bit_i, bit)| result | (bit << bit_i))
}

pub fn solve(lines: &[String]) -> Solution {
    let init: HashMap<&str, bool> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, value) = line.split_once(':').unwrap();
            (name.trim(), value.trim() == "1")
        })
        .collect();
    let gates: HashMap<&str, Gate> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (lhs, rhs) = line.split_once("->").unwrap();
            let (a, op_b) = lhs.split_once(' ').unwrap();
            let (op, b) = op_b.split_once(' ').unwrap();
            (
                rhs.trim(),
                Gate {
                    a: a.trim(),
                    b: b.trim(),
                    op: match op.trim() {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        _ => unreachable!(),
                    },
                },
            )
        })
        .collect();

    (solve_a(init, &gates).to_string(), "".to_string())
}
