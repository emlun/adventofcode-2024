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

use crate::{common::Solution, util::iter::WithSliding};

fn solve_a(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            report
                .iter()
                .sliding2()
                .map(|(a, b)| a - b)
                .all(|d| (-3..=-1).contains(&d))
                || report
                    .iter()
                    .sliding2()
                    .map(|(a, b)| a - b)
                    .all(|d| (1..=3).contains(&d))
        })
        .count()
}

fn solve_b(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                report
                    .iter()
                    .take(i)
                    .chain(report.iter().skip(i + 1))
                    .sliding2()
                    .map(|(a, b)| a - b)
                    .all(|d| (-3..=-1).contains(&d))
                    || report
                        .iter()
                        .take(i)
                        .chain(report.iter().skip(i + 1))
                        .sliding2()
                        .map(|(a, b)| a - b)
                        .all(|d| (1..=3).contains(&d))
            })
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let reports: Vec<Vec<i32>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    (solve_a(&reports).to_string(), solve_b(&reports).to_string())
}
