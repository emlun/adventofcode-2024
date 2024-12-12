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

use std::collections::{HashSet, VecDeque};

use crate::common::Solution;

#[derive(Debug)]
struct Tile {
    plant: char,
    neighbors: usize,
}

struct Map {
    rows: Vec<Vec<Tile>>,
    regions: Vec<HashSet<(usize, usize)>>,
}

fn chart(map: &[Vec<char>]) -> Map {
    let mut rows: Vec<Vec<Tile>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|plant| Tile {
                    plant: *plant,
                    neighbors: 0,
                })
                .collect()
        })
        .collect();
    let mut regions = Vec::new();
    let h = rows.len();
    let w = rows[0].len();

    let mut region_q = VecDeque::new();
    let mut perimeter_q = VecDeque::new();
    let mut visited = HashSet::new();
    perimeter_q.push_back((0, 0));
    while let Some((r, c)) = perimeter_q.pop_front() {
        if !visited.contains(&(r, c)) {
            let region_plant = rows[r][c].plant;
            let mut region = HashSet::new();
            region_q.push_back((r, c));

            while let Some((r, c)) = region_q.pop_front() {
                if !visited.contains(&(r, c)) {
                    if rows[r][c].plant == region_plant {
                        visited.insert((r, c));
                        region.insert((r, c));

                        for (rr, cc) in [
                            r.checked_sub(1).map(|rr| (rr, c)),
                            c.checked_sub(1).map(|cc| (r, cc)),
                            Some((r + 1, c)),
                            Some((r, c + 1)),
                        ]
                        .iter()
                        .flatten()
                        .copied()
                        .filter(|(rr, cc)| (0..h).contains(rr) && (0..w).contains(cc))
                        {
                            let neighbor = &mut rows[rr][cc];
                            if neighbor.plant == region_plant {
                                neighbor.neighbors += 1;
                            }
                            if !visited.contains(&(rr, cc)) {
                                region_q.push_back((rr, cc));
                            }
                        }
                    } else {
                        perimeter_q.push_back((r, c));
                    }
                }
            }
            regions.push(region);
        }
    }

    Map { rows, regions }
}

fn solve_a(chart: &Map) -> usize {
    chart
        .regions
        .iter()
        .map(|tiles| {
            tiles.len()
                * (4 * tiles.len()
                    - tiles
                        .iter()
                        .map(|(r, c)| chart.rows[*r][*c].neighbors)
                        .sum::<usize>())
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<char>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let chart = chart(&map);

    (solve_a(&chart).to_string(), "".to_string())
}
