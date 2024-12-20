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

#[derive(Eq, PartialEq)]
struct Game {
    walls: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn navigate(game: &Game) -> Vec<(usize, usize)> {
    let mut states = Vec::new();
    states.push(game.start);
    let walls = &game.walls;
    while states[states.len() - 1] != game.end {
        let pos = states[states.len() - 1];
        let (r, c) = pos;
        let next_pos = [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)]
            .into_iter()
            .find(|(rr, cc)| {
                (1..walls.len() - 1).contains(&rr)
                    && (1..walls[0].len() - 1).contains(&cc)
                    && states
                        .len()
                        .checked_sub(2)
                        .map(|i| states[i])
                        .map(|prev_pos| (*rr, *cc) != prev_pos)
                        .unwrap_or(true)
                    && !walls[*rr][*cc]
            })
            .unwrap();
        states.push(next_pos);
    }
    states
}

fn find_cheats(path: &[(usize, (usize, usize))], cheat_time: usize) -> usize {
    (0..path.len())
        .flat_map(|i| ((i + 1)..path.len()).map(move |j| (path[i], path[j])))
        .filter(|((ta, (ra, ca)), (tb, (rb, cb)))| {
            let dist = ra.abs_diff(*rb) + ca.abs_diff(*cb);
            dist <= cheat_time && tb - (ta + dist) >= 100
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let (walls, start, end) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (Vec::new(), (0, 0), (0, 0)),
            |(mut walls, start, end), (r, line)| {
                let (row, start, end) = line.chars().enumerate().fold(
                    (Vec::with_capacity(line.len()), start, end),
                    |(mut row, start, end), (c, ch)| {
                        row.push(ch == '#');
                        match ch {
                            'S' => (row, (r, c), end),
                            'E' => (row, start, (r, c)),
                            _ => (row, start, end),
                        }
                    },
                );
                walls.push(row);
                (walls, start, end)
            },
        );
    let game = Game { walls, start, end };
    let path: Vec<(usize, (usize, usize))> = navigate(&game).into_iter().enumerate().collect();

    (
        find_cheats(&path, 2).to_string(),
        find_cheats(&path, 20).to_string(),
    )
}
