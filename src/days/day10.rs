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

fn find_paths<'res>(
    map: &Vec<Vec<u8>>,
    (r, c): (usize, usize),
    peaks: &'res mut HashSet<(usize, usize)>,
) -> (&'res mut HashSet<(usize, usize)>, usize) {
    if map[r][c] == 9 {
        peaks.insert((r, c));
        (peaks, 1)
    } else {
        let paths = [
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
        .filter(|(rr, cc)| map[*rr][*cc] == map[r][c] + 1)
        .map(|(rr, cc)| {
            let (_, paths) = find_paths(map, (rr, cc), peaks);
            paths
        })
        .sum();
        (peaks, paths)
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let (map, heads): (Vec<Vec<u8>>, Vec<(usize, usize)>) = lines
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
                        tiles.push(elevation);
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

    let (scores, paths) = heads.iter().fold((0, 0), |(scores, paths), pos| {
        let mut peaks = HashSet::new();
        let (peaks, p) = find_paths(&map, *pos, &mut peaks);
        (scores + peaks.len(), paths + p)
    });

    (scores.to_string(), paths.to_string())
}
