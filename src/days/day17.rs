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

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn step(
    program: &[u8],
    ip: usize,
    mut a: u64,
    mut b: u64,
    mut c: u64,
) -> (usize, u64, u64, u64, Option<u8>) {
    let op = program[ip + 1];
    let literal_op: u64 = op.into();
    let combo_op = match op {
        0..=3 => op.into(),
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    };

    let mut output = None;

    let ip = match program[ip] {
        ADV => {
            a = a >> combo_op;
            ip + 2
        }
        BXL => {
            b ^= literal_op;
            ip + 2
        }
        BST => {
            b = combo_op % 8;
            ip + 2
        }
        JNZ => {
            if a == 0 {
                ip + 2
            } else {
                literal_op.try_into().unwrap()
            }
        }
        BXC => {
            b = b ^ c;
            ip + 2
        }
        OUT => {
            output = Some((combo_op % 8) as u8);
            ip + 2
        }
        BDV => {
            b = a >> combo_op;
            ip + 2
        }
        CDV => {
            c = a >> combo_op;
            ip + 2
        }
        _ => unreachable!(),
    };
    (ip, a, b, c, output)
}

fn solve_a(program: &[u8], mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;

    while ip < program.len() {
        let step_result = step(program, ip, a, b, c);
        (ip, a, b, c, _) = step_result;
        let (_, _, _, _, out) = step_result;
        output.extend(out);
    }
    output
}

fn solve_b(program: &[u8], b: u64, c: u64, find_output: &[u8]) -> u64 {
    assert_eq!(1, program.iter().step_by(2).filter(|i| **i == ADV).count());
    let adv_i = program
        .iter()
        .enumerate()
        .step_by(2)
        .find(|(_, i)| **i == ADV)
        .map(|(i, _)| i)
        .unwrap();
    let adv_bits = program[adv_i + 1];
    assert!((0..=3).contains(&adv_bits));

    let mut bit_segments = vec![0; find_output.len()];
    let mut i = find_output.len() - 1;
    loop {
        let a = bit_segments
            .iter()
            .rev()
            .copied()
            .fold(0, |a, bits| (a << adv_bits) | bits);
        let output = solve_a(program, a, b, c);
        if output.len() == find_output.len() && output[i..] == find_output[i..] {
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        } else {
            bit_segments[i] += 1;
        }
        if bit_segments[i] >= (1 << adv_bits) {
            bit_segments[i] = 0;
            bit_segments[i + 1] += 1;
            i += 1;
        }
    }
    let a = bit_segments
        .iter()
        .rev()
        .copied()
        .fold(0, |a, bits| (a << adv_bits) | bits);
    let output = solve_a(program, a, b, c);
    assert_eq!(output, find_output);
    a
}

pub fn solve(lines: &[String]) -> Solution {
    let (a, b, c) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map_while(|line| line.strip_prefix("Register"))
        .map(|line| line.trim())
        .fold((0, 0, 0), |(a, b, c), line| {
            if let Some(a) = line.strip_prefix("A:") {
                (a.trim().parse().unwrap(), b, c)
            } else if let Some(b) = line.strip_prefix("B:") {
                (a, b.trim().parse().unwrap(), c)
            } else if let Some(c) = line.strip_prefix("C:") {
                (a, b, c.trim().parse().unwrap())
            } else {
                unreachable!()
            }
        });

    let program: Vec<u8> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .skip_while(|line| line.starts_with("Register"))
        .flat_map(|line| line.strip_prefix("Program:"))
        .flat_map(|line| line.trim().split(','))
        .map(|s| s.parse().unwrap())
        .collect();

    let solution_a = solve_a(&program, a, b, c);
    let solution_b = solve_b(&program, b, c, &program);
    (
        solution_a
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(","),
        solution_b.to_string(),
    )
}
