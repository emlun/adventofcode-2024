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

use crate::{
    common::Solution,
    search::astar::{self, astar},
};

const GOAL: (usize, usize) = (70, 70);
const T: usize = 1024;

#[derive(Eq, PartialEq)]
struct Game<'walls> {
    walls: &'walls HashMap<(usize, usize), usize>,
    start: (usize, usize),
    end: (usize, usize),
    t: usize,
}

#[derive(Eq, PartialEq)]
struct State<'game> {
    game: &'game Game<'game>,
    pos: (usize, usize),
    steps: usize,
}

impl<'game> astar::State for State<'game> {
    type DuplicationKey = (usize, usize);
    type Value = usize;
    type NewStates = Box<dyn Iterator<Item = Self> + 'game>;

    fn value(&self) -> Self::Value {
        self.steps
    }

    fn estimate(&self) -> Self::Value {
        let (er, ec) = self.game.end;
        let (r, c) = self.pos;
        self.value() + r.abs_diff(er) + c.abs_diff(ec)
    }

    fn duplication_key(&self) -> Self::DuplicationKey {
        self.pos
    }

    fn generate_moves(self) -> Self::NewStates {
        let (r, c) = self.pos;
        Box::new(
            [
                Some((r + 1, c)),
                Some((r, c + 1)),
                r.checked_sub(1).map(|rr| (rr, c)),
                c.checked_sub(1).map(|cc| (r, cc)),
            ]
            .into_iter()
            .flatten()
            .filter(|pos| {
                let (rr, cc) = *pos;
                let (er, ec) = self.game.end;
                rr <= er
                    && cc <= ec
                    && self
                        .game
                        .walls
                        .get(pos)
                        .map(|wt| *wt >= self.game.t)
                        .unwrap_or(true)
            })
            .map(move |pos| State {
                pos,
                steps: self.steps + 1,
                ..self
            }),
        )
    }
}

fn solve_b(walls: &HashMap<(usize, usize), usize>) -> (usize, usize) {
    let mut t_min = T;
    let mut t_max = walls.len();
    let tb = loop {
        if t_min == t_max {
            break t_min;
        }
        let t = (t_min + t_max) / 2;
        let game = Game {
            walls,
            start: (0, 0),
            end: GOAL,
            t: t + 1,
        };
        if astar(State {
            game: &game,
            pos: game.start,
            steps: 0,
        })
        .is_some()
        {
            t_min = t + 1;
        } else {
            t_max = t;
        }
    };
    *walls
        .iter()
        .find_map(|(pos, t)| if *t == tb { Some(pos) } else { None })
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let walls = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(t, line)| {
            let (xs, ys) = line.split_once(',').unwrap();
            ((xs.parse().unwrap(), ys.parse().unwrap()), t)
        })
        .collect();
    let game = Game {
        walls: &walls,
        start: (0, 0),
        end: GOAL,
        t: T,
    };

    let solution_a = astar(State {
        game: &game,
        pos: game.start,
        steps: 0,
    })
    .unwrap()
    .steps;

    let (bx, by) = solve_b(&walls);

    (solution_a.to_string(), format!("{},{}", bx, by))
}
