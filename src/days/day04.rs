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

fn solve_a(grid: &[Vec<char>]) -> usize {
    (0..grid.len())
        .flat_map(|r| (0..grid[r].len()).map(move |c| (r as isize, c as isize)))
        .map(|(r, c)| {
            (-1_isize..=1)
                .flat_map(|dr| (-1_isize..=1).map(move |dc| (dr, dc)))
                .filter(|drc| *drc != (0, 0))
                .filter(|(dr, dc)| {
                    (0..grid.len() as isize).contains(&(r + 3 * dr))
                        && (0..grid[0].len() as isize).contains(&(c + 3 * dc))
                })
                .filter(|(dr, dc)| {
                    grid[r as usize][c as usize] == 'X'
                        && grid[(r + dr) as usize][(c + dc) as usize] == 'M'
                        && grid[(r + 2 * dr) as usize][(c + 2 * dc) as usize] == 'A'
                        && grid[(r + 3 * dr) as usize][(c + 3 * dc) as usize] == 'S'
                })
                .count()
        })
        .sum()
}

fn solve_b(grid: &[Vec<char>]) -> usize {
    (1..grid.len() - 1)
        .flat_map(|r| (1..grid[r].len() - 1).map(move |c| (r, c)))
        .filter(|(r, c)| {
            let tl = grid[r - 1][c - 1];
            grid[*r][*c] == 'A' && {
                let tr = grid[r - 1][c + 1];
                let bl = grid[r + 1][c - 1];
                let br = grid[r + 1][c + 1];

                ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
                    && ((bl == 'M' && tr == 'S') || (bl == 'S' && tr == 'M'))
            }
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    (solve_a(&grid).to_string(), solve_b(&grid).to_string())
}
