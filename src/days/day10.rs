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

#[derive(Clone, Debug)]
struct Tile {
    elevation: u8,
    trails: usize,
}

fn find_peaks<'res>(
    map: &mut Vec<Vec<Tile>>,
    (r, c): (usize, usize),
    result: &'res mut HashSet<(usize, usize)>,
) -> &'res mut HashSet<(usize, usize)> {
    if map[r][c].elevation == 9 {
        result.insert((r, c));
        result
    } else {
        for (rr, cc) in [
            r.checked_sub(1).map(|rr| (rr, c)),
            c.checked_sub(1).map(|cc| (r, cc)),
            Some(r + 1).filter(|rr| *rr < map.len()).map(|rr| (rr, c)),
            Some(c + 1)
                .filter(|cc| *cc < map[0].len())
                .map(|cc| (r, cc)),
        ]
        .iter()
        .flatten()
        .copied()
        {
            if map[rr][cc].elevation == map[r][c].elevation + 1 {
                find_peaks(map, (rr, cc), result);
            }
        }
        result
    }
}

fn find_paths(map: &mut Vec<Vec<Tile>>, (r, c): (usize, usize)) -> usize {
    if map[r][c].elevation == 9 {
        1
    } else {
        let trails: usize = [
            r.checked_sub(1).map(|rr| (rr, c)),
            c.checked_sub(1).map(|cc| (r, cc)),
            Some(r + 1).filter(|rr| *rr < map.len()).map(|rr| (rr, c)),
            Some(c + 1)
                .filter(|cc| *cc < map[0].len())
                .map(|cc| (r, cc)),
        ]
        .iter()
        .flatten()
        .copied()
        .map(|(rr, cc)| {
            if map[rr][cc].elevation == map[r][c].elevation + 1 {
                if map[rr][cc].trails > 0 {
                    map[rr][cc].trails
                } else {
                    find_paths(map, (rr, cc))
                }
            } else {
                0
            }
        })
        .sum();
        map[r][c].trails += trails;
        trails
    }
}

fn solve_a(mut map: Vec<Vec<Tile>>, heads: &[(usize, usize)]) -> usize {
    heads
        .iter()
        .map(|pos| find_peaks(&mut map, *pos, &mut HashSet::new()).len())
        .sum()
}

fn solve_b(mut map: Vec<Vec<Tile>>, heads: &[(usize, usize)]) -> usize {
    heads.iter().map(|pos| find_paths(&mut map, *pos)).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (map, heads): (Vec<Vec<Tile>>, Vec<(usize, usize)>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (Vec::with_capacity(lines.len()), Vec::new()),
            |(mut rows, mut heads), (r, line)| {
                let (tiles, row_heads) = line.chars().enumerate().fold(
                    (Vec::with_capacity(line.len()), Vec::new()),
                    |(mut tiles, mut heads), (c, ch)| {
                        let elevation = ch.to_digit(10).unwrap() as u8;
                        tiles.push(Tile {
                            elevation,
                            trails: 0,
                        });
                        if elevation == 0 {
                            heads.push((r, c));
                        }
                        (tiles, heads)
                    },
                );
                rows.push(tiles);
                heads.extend(row_heads);
                (rows, heads)
            },
        );

    (
        solve_a(map.clone(), &heads).to_string(),
        solve_b(map.clone(), &heads).to_string(),
    )
}
