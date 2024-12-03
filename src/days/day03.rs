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

fn eval_muls(s: &str) -> i32 {
    s.split("mul(")
        .skip(1)
        .filter_map(|segment| {
            let mut it = segment.split(',');
            let first_num = it.next().and_then(|s| s.parse::<i32>().ok())?;
            let second_num = it
                .next()
                .and_then(|s| s.split_inclusive(')').next())
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.parse::<i32>().ok())?;
            Some(first_num * second_num)
        })
        .sum()
}

fn solve_a(lines: &[String]) -> i32 {
    lines.iter().map(|line| eval_muls(line)).sum()
}

fn solve_b(lines: &[String]) -> i32 {
    let (sum, _) = lines
        .iter()
        .flat_map(|line| line.split_inclusive("do()"))
        .flat_map(|segment| segment.split_inclusive("don't()"))
        .fold((0, true), |(sum, enabled), segment| {
            (
                if enabled {
                    sum + eval_muls(segment)
                } else {
                    sum
                },
                (enabled && !segment.ends_with("don't()"))
                    || (!enabled && segment.ends_with("do()")),
            )
        });
    sum
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
