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

use std::{collections::HashSet, rc::Rc};

use crate::{
    common::Solution,
    search::astar::{self, astar_all_best},
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
    prev: Option<Rc<Self>>,
    pos: (usize, usize),
    dir: u8,
    score: usize,
}

impl<'game> State<'game> {
    fn path(&self) -> Vec<(usize, usize)> {
        let mut path = self
            .prev
            .as_ref()
            .map(|prev| prev.path())
            .unwrap_or_default();
        path.push(self.pos);
        path
    }
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
        let prev = Rc::new(self);
        Box::new(
            [
                Self {
                    dir: (prev.dir + 1) % 4,
                    score: prev.score + 1000,
                    prev: Some(Rc::clone(&prev)),
                    ..*prev
                },
                Self {
                    dir: (prev.dir + 3) % 4,
                    score: prev.score + 1000,
                    prev: Some(Rc::clone(&prev)),
                    ..*prev
                },
                Self {
                    game: prev.game,
                    pos: match prev.dir {
                        0 => (r - 1, c),
                        1 => (r, c + 1),
                        2 => (r + 1, c),
                        3 => (r, c - 1),
                        _ => unreachable!(),
                    },
                    score: prev.score + 1,
                    dir: prev.dir,
                    prev: Some(prev),
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

    let paths = astar_all_best(State {
        game: &game,
        prev: None,
        pos: game.start,
        dir: 1,
        score: 0,
    });
    let solution_a = paths[0].score;
    let tiles =
        paths
            .into_iter()
            .map(|last| last.path())
            .fold(HashSet::new(), |mut tiles, path| {
                tiles.extend(path);
                tiles
            });
    let solution_b = tiles.len();

    (solution_a.to_string(), solution_b.to_string())
}
