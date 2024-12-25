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

pub fn solve(lines: &[String]) -> Solution {
    let (locks, keys, _): (Vec<[u8; 5]>, Vec<[u8; 5]>, _) =
        lines.iter().filter(|line| !line.is_empty()).fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut locks, mut keys, mut buf), line| {
                buf.push(line);
                if buf.len() == 7 {
                    let cols: [u8; 5] = [0, 1, 2, 3, 4].map(|c| {
                        buf.iter()
                            .filter(|col| col.chars().nth(c).unwrap() == '#')
                            .count() as u8
                    });
                    if buf[0] == "#####" {
                        locks.push(cols);
                    } else if buf.last().map(|s| s.as_str()) == Some("#####") {
                        keys.push(cols);
                    } else {
                        unreachable!()
                    }
                    buf.clear();
                }
                (locks, keys, buf)
            },
        );

    let solution_a: usize = locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 7))
                .count()
        })
        .sum();

    (solution_a.to_string(), "".to_string())
}
