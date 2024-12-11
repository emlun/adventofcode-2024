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

use std::collections::{HashMap, HashSet};

use crate::common::Solution;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    r: usize,
    c: usize,
    dir: u8,
}

struct Map {
    obstacles_r: Vec<Vec<usize>>,
    obstacles_c: Vec<Vec<usize>>,
}

impl Map {
    fn new(map: &[Vec<bool>]) -> Self {
        let obstacles_r = map
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell)
                    .map(|(c, _)| c)
                    .collect()
            })
            .collect();
        let obstacles_c = (0..map[0].len())
            .map(|c| (0..map.len()).filter(|r| map[*r][c]).collect())
            .collect();
        Self {
            obstacles_r,
            obstacles_c,
        }
    }

    fn with_obstacle(&self, r: usize, c: usize) -> Self {
        Self {
            obstacles_r: {
                let mut o = self.obstacles_r.clone();
                o[r].push(c);
                o[r].sort();
                o
            },
            obstacles_c: {
                let mut o = self.obstacles_c.clone();
                o[c].push(r);
                o[c].sort();
                o
            },
        }
    }

    fn seek(obstacles_r: &[Vec<usize>], obstacles_c: &[Vec<usize>], pos: &Pos) -> Option<Pos> {
        let (rr, cc) = match pos.dir {
            0 => {
                let ri = obstacles_c[pos.c].partition_point(|rr| *rr < pos.r);
                let rr = obstacles_c[pos.c].get(ri.checked_sub(1)?)? + 1;
                Some((rr, pos.c)).filter(|(rr, _)| *rr <= pos.r)
            }
            1 => {
                let ci = obstacles_r[pos.r].partition_point(|cc| *cc <= pos.c);
                let cc = obstacles_r[pos.r].get(ci)? - 1;
                Some((pos.r, cc)).filter(|(_, cc)| *cc >= pos.c)
            }
            2 => {
                let ri = obstacles_c[pos.c].partition_point(|rr| *rr <= pos.r);
                let rr = obstacles_c[pos.c].get(ri)? - 1;
                Some((rr, pos.c)).filter(|(rr, _)| *rr >= pos.r)
            }
            3 => {
                let ci = obstacles_r[pos.r].partition_point(|cc| *cc < pos.c);
                let cc = obstacles_r[pos.r].get(ci.checked_sub(1)?)? + 1;
                Some((pos.r, cc)).filter(|(_, cc)| *cc <= pos.c)
            }
            _ => unreachable!(),
        }?;
        Some(Pos {
            r: rr,
            c: cc,
            dir: (pos.dir + 1) % 4,
        })
    }
}

fn step(Pos { r, c, dir }: &Pos) -> Option<Pos> {
    let (rr, cc) = match dir {
        0 => (r.checked_sub(1)?, *c),
        1 => (*r, c + 1),
        2 => (r + 1, *c),
        3 => (*r, c.checked_sub(1)?),
        _ => unreachable!(),
    };
    Some(Pos {
        r: rr,
        c: cc,
        dir: *dir,
    })
}

fn trace_path(map: &[Vec<bool>], start: Pos) -> (Vec<Pos>, bool) {
    let mut is_loop = false;
    let mut visited = HashSet::new();
    (
        std::iter::successors(Some(start), |pos @ Pos { r, c, dir }| {
            if visited.contains(pos) {
                is_loop = true;
                None
            } else {
                visited.insert(*pos);
                let Pos { r: rr, c: cc, .. } = step(pos)?;
                Some(if *map.get(rr)?.get(cc)? {
                    Pos {
                        r: *r,
                        c: *c,
                        dir: (dir + 1) % 4,
                    }
                } else {
                    Pos {
                        r: rr,
                        c: cc,
                        dir: *dir,
                    }
                })
            }
        })
        .collect(),
        is_loop,
    )
}

fn is_loop(map: &Map, start: Pos) -> bool {
    let mut visited = HashSet::new();
    let mut pos = start;
    while let Some(new_pos) = Map::seek(&map.obstacles_r, &map.obstacles_c, &pos) {
        if visited.contains(&new_pos) {
            return true;
        } else {
            visited.insert(new_pos);
            pos = new_pos;
        }
    }
    false
}

fn solve_a(path: &[Pos]) -> usize {
    path.iter()
        .map(|Pos { r, c, .. }| (r, c))
        .collect::<HashSet<_>>()
        .len()
}

fn solve_b(map: Vec<Vec<bool>>, start: Pos, path: &[Pos]) -> usize {
    let obstacle_map = Map::new(&map);
    let candidate_coords: HashSet<(usize, usize)> = path
        .iter()
        .flat_map(step)
        .filter(|Pos { r, c, .. }| *r < map.len() && *c < map[0].len())
        .map(|Pos { r, c, .. }| (r, c))
        .collect();
    candidate_coords
        .into_iter()
        .filter(|(r, c)| {
            if !map[*r][*c] {
                is_loop(&obstacle_map.with_obstacle(*r, *c), start)
            } else {
                false
            }
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let (start, map): (Option<Pos>, Vec<Vec<bool>>) = lines
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
                                start = Some(Pos { r, c, dir: 0 });
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
