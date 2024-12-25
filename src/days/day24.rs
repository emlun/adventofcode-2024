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

use std::collections::{BTreeSet, HashMap};

use crate::common::Solution;

#[derive(Debug, Eq, PartialEq)]
struct Gate<'gate> {
    a: &'gate str,
    b: &'gate str,
    op: Op,
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

fn assemble(state: &HashMap<&str, bool>, prefix: char) -> u64 {
    state
        .iter()
        .flat_map(|(k, v)| {
            let bit_i: u64 = k.strip_prefix(prefix)?.parse().ok()?;
            Some((bit_i, if *v { 1 } else { 0 }))
        })
        .fold(0, |result, (bit_i, bit)| result | (bit << bit_i))
}

fn solve_a<'gate>(
    mut state: HashMap<&'gate str, bool>,
    gates: &HashMap<&'gate str, Gate>,
    swaps: &HashMap<&'gate str, &'gate str>,
) -> Option<u64> {
    while !gates
        .keys()
        .filter(|k| k.starts_with('z'))
        .all(|k| state.contains_key(k))
    {
        let mut infinite = true;
        for (rhs, lhs) in gates {
            let rhs = swaps.get(rhs).unwrap_or(rhs);
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
                    infinite = false;
                }
            }
        }
        if infinite {
            return None;
        }
    }

    Some(assemble(&state, 'z'))
}

fn solve_b<'gate>(init: HashMap<&'gate str, bool>, gates: &HashMap<&'gate str, Gate>) -> String {
    assert_eq!(gates.len(), (init.len() / 2 - 1) * 5 + 2);
    let l = init.len() / 2;
    let x: Vec<String> = (0..(l + 1)).map(|s| format!("x{:02}", s)).collect();
    let y: Vec<String> = (0..(l + 1)).map(|s| format!("y{:02}", s)).collect();
    let z: Vec<String> = (0..(l + 1)).map(|s| format!("z{:02}", s)).collect();

    let mut wrong: BTreeSet<&str> = BTreeSet::new();

    {
        match gates["z00"] {
            Gate {
                a: "x00",
                b: "y00",
                op: Op::Xor,
            } => {}
            _ => {
                wrong.insert("z00");
            }
        };
    }

    for i in 1..l {
        let zi = z[i].as_str();
        let gate_zi = &gates[&zi];
        let (si, gate_si) = gates
            .iter()
            .find(|(_, gate)| {
                **gate
                    == Gate {
                        a: &x[i],
                        b: &y[i],
                        op: Op::Xor,
                    }
            })
            .unwrap();

        if gate_zi.op == Op::Xor {
            if gate_zi.a == *si {
                gate_zi.b
            } else if gate_zi.b == *si {
                gate_zi.a
            } else if gates[gate_zi.a].op == Op::Or {
                wrong.insert(gate_zi.b);
                wrong.insert(si);
                continue;
            } else if gates[gate_zi.b].op == Op::Or {
                wrong.insert(gate_zi.a);
                wrong.insert(si);
                continue;
            } else {
                dbg!(zi, gate_zi, si, gate_si);
                todo!()
            };
        } else {
            let (real_zi, _) = gates
                .iter()
                .find(|(_, gate)| gate.op == Op::Xor && (gate.a == *si || gate.b == *si))
                .unwrap();
            wrong.insert(zi);
            wrong.insert(real_zi);
        }
    }

    assert_eq!(wrong.len(), 8);
    let wrong = wrong.into_iter().collect::<Vec<_>>();
    wrong.join(",")
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
            let (a, b) = (a.trim(), b.trim());
            let (a, b) = (std::cmp::min(a, b), std::cmp::max(a, b));
            (
                rhs.trim(),
                Gate {
                    a,
                    b,
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

    (
        solve_a(init.clone(), &gates, &HashMap::new())
            .unwrap()
            .to_string(),
        solve_b(init, &gates).to_string(),
    )
}
