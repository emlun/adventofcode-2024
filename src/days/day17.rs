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

fn solve_a(program: &[u8], mut a: u64, mut b: u64, mut c: u64) -> Vec<String> {
    let mut output = Vec::new();
    let mut ip = 0;

    const ADV: u8 = 0;
    const BXL: u8 = 1;
    const BST: u8 = 2;
    const JNZ: u8 = 3;
    const BXC: u8 = 4;
    const OUT: u8 = 5;
    const BDV: u8 = 6;
    const CDV: u8 = 7;

    while ip < program.len() {
        let op = program[ip + 1];
        let literal_op: u64 = op.into();
        let combo_op = match op {
            0..=3 => op.into(),
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        ip = match program[ip] {
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
                output.push((combo_op % 8).to_string());
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
    }
    output
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

    (
        solve_a(&program, a, b, c).join(",").to_string(),
        "".to_string(),
    )
}
