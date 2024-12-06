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

use std::collections::HashSet;

use crate::common::Solution;

fn step(r: usize, c: usize, dir: u8) -> Option<(usize, usize)> {
    Some(match dir {
        0 => (r.checked_sub(1)?, c),
        1 => (r, c + 1),
        2 => (r + 1, c),
        3 => (r, c.checked_sub(1)?),
        _ => unreachable!(),
    })
}

fn trace_path(map: &[Vec<bool>], start: (usize, usize, u8)) -> (Vec<(usize, usize, u8)>, bool) {
    let mut is_loop = false;
    let mut visited = HashSet::new();
    (
        std::iter::successors(Some(start), |(r, c, dir)| {
            if visited.contains(&(*r, *c, *dir)) {
                is_loop = true;
                None
            } else {
                visited.insert((*r, *c, *dir));
                let (rr, cc) = step(*r, *c, *dir)?;
                Some(if *map.get(rr)?.get(cc)? {
                    (*r, *c, (dir + 1) % 4)
                } else {
                    (rr, cc, *dir)
                })
            }
        })
        .collect(),
        is_loop,
    )
}

fn solve_a(path: &[(usize, usize, u8)]) -> usize {
    path.iter()
        .map(|(r, c, _)| (r, c))
        .collect::<HashSet<_>>()
        .len()
}

fn solve_b(
    mut map: Vec<Vec<bool>>,
    start: (usize, usize, u8),
    path: &[(usize, usize, u8)],
) -> usize {
    let candidate_coords: HashSet<(usize, usize)> = path
        .iter()
        .flat_map(|(r, c, dir)| step(*r, *c, *dir))
        .filter(|(r, c)| *r < map.len() && *c < map[0].len())
        .collect();
    candidate_coords
        .into_iter()
        .filter(|(r, c)| {
            if !map[*r][*c] {
                map[*r][*c] = true;
                let (_, is_loop) = trace_path(&map, start);
                map[*r][*c] = false;
                is_loop
            } else {
                false
            }
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let (start, map): (Option<(usize, usize, u8)>, Vec<Vec<bool>>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (None, Vec::with_capacity(lines.len())),
            |(mut start, mut map), (r, line)| {
                map.push(
                    line.chars()
                        .enumerate()
                        .map(|(c, ch)| match ch {
                            '#' => true,
                            '^' => {
                                start = Some((r, c, 0));
                                false
                            }
                            _ => false,
                        })
                        .collect(),
                );
                (start, map)
            },
        );
    let start = start.expect("Failed to find start position");

    let (path, _) = trace_path(&map, start);

    (
        solve_a(&path).to_string(),
        solve_b(map, start, &path).to_string(),
    )
}
