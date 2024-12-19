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
struct Game<'pat> {
    patterns: &'pat [&'pat str],
    goal: &'pat str,
}

#[derive(Eq, PartialEq)]
struct State<'game> {
    game: &'game Game<'game>,
    prefix: String,
}

fn count_solutions<'game>(
    game: &Game<'game>,
    state: State,
    memo: &mut HashMap<&'game str, usize>,
) -> usize {
    if let Some(rest) = game.goal.strip_prefix(&state.prefix) {
        if let Some(m) = memo.get(rest) {
            *m
        } else if rest.is_empty() {
            1
        } else {
            let solutions = game
                .patterns
                .iter()
                .map(|pat| {
                    count_solutions(
                        game,
                        State {
                            game,
                            prefix: format!("{}{}", state.prefix, pat),
                        },
                        memo,
                    )
                })
                .sum();
            memo.insert(rest, solutions);
            solutions
        }
    } else {
        0
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let patterns: Vec<&str> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.split(','))
        .map(|s| s.trim())
        .collect();

    let goals: Vec<&str> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .map(|line| line.as_str())
        .collect();

    let (solution_a, solution_b): (usize, usize) = goals.iter().fold((0, 0), |(a, b), goal| {
        let game = Game {
            patterns: &patterns,
            goal,
        };
        let mut memo = HashMap::new();
        let sol = count_solutions(
            &game,
            State {
                game: &game,
                prefix: "".to_string(),
            },
            &mut memo,
        );

        (a + if sol > 0 { 1 } else { 0 }, b + sol)
    });

    (solution_a.to_string(), solution_b.to_string())
}
