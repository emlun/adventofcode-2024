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

use std::collections::HashMap;

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

fn find_cheats(
    uncheat_path: &HashMap<(usize, usize), usize>,
    pos: (usize, usize),
    cheated_time: usize,
) -> Vec<((usize, usize), usize)> {
    if cheated_time < 2 {
        let (r, c) = pos;
        [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)]
            .into_iter()
            .filter(|(rr, cc)| {
                *rr >= 1
                    && *cc >= 1
                    && (uncheat_path.get(&(*rr, *cc)).is_some() || cheated_time <= 2)
            })
            .flat_map(|pos| find_cheats(uncheat_path, pos, cheated_time + 1))
            .collect()
    } else {
        vec![(pos, cheated_time)]
    }
}

fn solve_a(game: &Game) -> usize {
    let path = navigate(game);
    let time_to: HashMap<(usize, usize), usize> =
        path.iter().enumerate().map(|(t, pos)| (*pos, t)).collect();

    path.into_iter()
        .flat_map(|cheat_start| {
            let time_to = &time_to;
            find_cheats(&time_to, cheat_start, 0)
                .into_iter()
                .filter(move |(cheat_end, _)| *cheat_end != cheat_start)
                .flat_map(move |(cheat_end, cheated_time)| {
                    let t0 = time_to[&cheat_start];
                    let t1 = time_to.get(&cheat_end)?;
                    let dt = t1.saturating_sub(t0 + cheated_time);
                    Some(dt).filter(|dt| *dt >= 100)
                })
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

    (solve_a(&game).to_string(), "".to_string())
}
