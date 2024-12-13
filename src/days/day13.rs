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

#[cfg(debug_assertions)]
use crate::util::gcd;

struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn solve_a(games: &[Game]) -> i64 {
    games
        .iter()
        .map(
            |Game {
                 a: (xa, ya),
                 b: (xb, yb),
                 prize: (xp, yp),
             }| {
                #[cfg(debug_assertions)]
                if xa * yb == xb * ya {
                    let ga = gcd(*xa as usize, *ya as usize) as i64;
                    let gb = gcd(*xb as usize, *yb as usize) as i64;
                    let gp = gcd(*xp as usize, *yp as usize) as i64;
                    let na = (xa / ga, ya / ga);
                    let nb = (xb / gb, yb / gb);
                    let np = (xp / gp, yp / gp);
                    debug_assert_eq!(na, nb, "System matrix columns are not colinear");
                    debug_assert_eq!(na, np, "RHS is colinear with system matrix");
                    return 0;
                }

                let b = (xa * yp - xp * ya) / (xa * yb - xb * ya);
                let a = (xp - xb * (xa * yp - xp * ya) / (xa * yb - xb * ya)) / xa;
                if (xa * a + xb * b == *xp) && (ya * a + yb * b == *yp) {
                    3 * a + b
                } else {
                    0
                }
            },
        )
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    fn parse_line(prefix: &str, op: &str, line: &str) -> Option<(i64, i64)> {
        let (xs, ys) = line
            .trim()
            .strip_prefix(prefix)?
            .strip_prefix(": X")?
            .strip_prefix(op)?
            .split_once(',')?;
        Some((
            xs.parse().ok()?,
            ys.strip_prefix(" Y")?.strip_prefix(op)?.parse().ok()?,
        ))
    }

    let (games, _, _) = lines.iter().filter(|line| !line.is_empty()).fold(
        (Vec::with_capacity((lines.len() + 1) / 4), None, None),
        |(mut games, a, b), line| match (a, b) {
            (None, None) => (
                games,
                Some(parse_line("Button A", "+", line).unwrap()),
                None,
            ),
            (Some(a), None) => (
                games,
                Some(a),
                Some(parse_line("Button B", "+", line).unwrap()),
            ),
            (Some(a), Some(b)) => {
                games.push(Game {
                    a,
                    b,
                    prize: parse_line("Prize", "=", line).unwrap(),
                });
                (games, None, None)
            }
            _ => unreachable!(),
        },
    );

    (
        solve_a(&games).to_string(),
        solve_a(
            &games
                .into_iter()
                .map(|game| Game {
                    prize: (game.prize.0 + 10000000000000, game.prize.1 + 10000000000000),
                    ..game
                })
                .collect::<Vec<_>>(),
        )
        .to_string(),
    )
}
