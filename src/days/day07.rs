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

type Equation = (u64, Vec<u64>);

fn concat(a: u64, b: u64) -> u64 {
    if b == 0 {
        a * 10
    } else {
        a * 10_u64.pow(1 + b.ilog10()) + b
    }
}

fn can_solve<const CONCAT: bool>(lhs: u64, acc: u64, rhs: &[u64]) -> bool {
    if let Some((head, tail)) = rhs.split_first() {
        can_solve::<{ CONCAT }>(lhs, acc + head, tail)
            || can_solve::<{ CONCAT }>(lhs, acc * head, tail)
            || (CONCAT && can_solve::<{ CONCAT }>(lhs, concat(acc, *head), tail))
    } else {
        lhs == acc
    }
}

fn solve_a(equations: &[Equation]) -> (Vec<&Equation>, Vec<&Equation>) {
    equations
        .iter()
        .partition(|(lhs, rhs)| can_solve::<false>(*lhs, rhs[0], &rhs[1..]))
}

fn solve_b(equations: &[&Equation]) -> u64 {
    equations
        .iter()
        .filter(|(lhs, rhs)| can_solve::<true>(*lhs, rhs[0], &rhs[1..]))
        .map(|(lhs, _)| lhs)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let equations: Vec<Equation> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (lhs, rhs) = line.split_once(':').unwrap();
            (
                lhs.parse().unwrap(),
                rhs.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect();

    let (sol, unsol) = solve_a(&equations);
    let solution_a: u64 = sol.iter().map(|(lhs, _)| lhs).sum();

    (
        solution_a.to_string(),
        (solution_a + solve_b(&unsol)).to_string(),
    )
}
