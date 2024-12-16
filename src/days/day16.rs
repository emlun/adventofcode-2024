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

use crate::{
    common::Solution,
    search::astar::{self, astar},
};

#[derive(Eq, PartialEq)]
struct Game {
    walls: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Eq, PartialEq)]
struct State<'game> {
    game: &'game Game,
    pos: (usize, usize),
    dir: u8,
    score: usize,
}

impl<'game> astar::State for State<'game> {
    type DuplicationKey = ((usize, usize), u8);
    type Value = usize;
    type NewStates = Box<dyn Iterator<Item = Self> + 'game>;

    fn value(&self) -> Self::Value {
        self.score
    }

    fn estimate(&self) -> Self::Value {
        let (er, ec) = self.game.end;
        let (r, c) = self.pos;
        self.value() + r.abs_diff(er) + c.abs_diff(ec)
    }

    fn duplication_key(&self) -> Self::DuplicationKey {
        (self.pos, self.dir)
    }

    fn generate_moves(self) -> Self::NewStates {
        let (r, c) = self.pos;
        Box::new(
            [
                Self {
                    dir: (self.dir + 1) % 4,
                    score: self.score + 1000,
                    ..self
                },
                Self {
                    dir: (self.dir + 3) % 4,
                    score: self.score + 1000,
                    ..self
                },
                Self {
                    pos: match self.dir {
                        0 => (r - 1, c),
                        1 => (r, c + 1),
                        2 => (r + 1, c),
                        3 => (r, c - 1),
                        _ => unreachable!(),
                    },
                    score: self.score + 1,
                    ..self
                },
            ]
            .into_iter()
            .filter(|state| {
                let (rr, cc) = state.pos;
                !state.game.walls[rr][cc]
            }),
        )
    }
}

fn solve_a(game: &Game) -> usize {
    astar(State {
        game,
        pos: game.start,
        dir: 1,
        score: 0,
    })
    .unwrap()
    .score
}

pub fn solve(lines: &[String]) -> Solution {
    let (walls, start, end): (Vec<Vec<bool>>, (usize, usize), (usize, usize)) = lines
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
