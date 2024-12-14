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

use std::cmp::Ordering;

use crate::common::Solution;

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

const W: i64 = 101;
const H: i64 = 103;
const PICTURE_W: u64 = 31;
const PICTURE_H: u64 = 33;

fn solve_a(robots: &[Robot], steps: i64) -> i64 {
    let (q1, q2, q3, q4) = robots.iter().fold(
        (0, 0, 0, 0),
        |(q1, q2, q3, q4),
         Robot {
             p: (px, py),
             v: (vx, vy),
         }| {
            let x = (px + vx * steps).rem_euclid(W);
            let y = (py + vy * steps).rem_euclid(H);
            match (x.cmp(&(W / 2)), y.cmp(&(H / 2))) {
                (Ordering::Equal, _) | (_, Ordering::Equal) => (q1, q2, q3, q4),
                (Ordering::Greater, Ordering::Greater) => (q1 + 1, q2, q3, q4),
                (Ordering::Less, Ordering::Greater) => (q1, q2 + 1, q3, q4),
                (Ordering::Less, Ordering::Less) => (q1, q2, q3 + 1, q4),
                (Ordering::Greater, Ordering::Less) => (q1, q2, q3, q4 + 1),
            }
        },
    );
    q1 * q2 * q3 * q4
}

fn solve_b(robots: &[Robot]) -> i64 {
    for step in 1..100_000 {
        let (xs, ys): (Vec<i64>, Vec<i64>) = robots.iter().fold(
            (
                Vec::with_capacity(robots.len()),
                Vec::with_capacity(robots.len()),
            ),
            |(mut xs, mut ys),
             Robot {
                 p: (px, py),
                 v: (vx, vy),
             }| {
                let x = (px + vx * step).rem_euclid(W);
                let y = (py + vy * step).rem_euclid(H);
                xs.push(x);
                ys.push(y);
                (xs, ys)
            },
        );

        #[cfg(not(debug_assertions))]
        let mut xs = xs;
        #[cfg(not(debug_assertions))]
        let mut ys = ys;

        let (median_x, median_y) = {
            #[cfg(debug_assertions)]
            let mut xs = xs.clone();
            #[cfg(debug_assertions)]
            let mut ys = ys.clone();

            xs.sort();
            ys.sort();
            (xs[xs.len() / 2], ys[ys.len() / 2])
        };

        if xs
            .iter()
            .zip(ys.iter())
            .filter(|(x, y)| {
                x.abs_diff(median_x) <= PICTURE_W / 2 + 1
                    && y.abs_diff(median_y) <= PICTURE_H / 2 + 1
            })
            .count()
            >= robots.len() * 70 / 100
        {
            #[cfg(debug_assertions)]
            {
                let mut grid: Vec<Vec<u8>> = vec![vec![0; W as usize]; H as usize];
                for (x, y) in xs.iter().zip(ys.iter()) {
                    grid[*y as usize][*x as usize] += 1;
                }
                println!("Step {}:", step);
                for row in grid {
                    println!(
                        "{}",
                        row.into_iter()
                            .map(|count| if count > 0 {
                                count.to_string()
                            } else {
                                ' '.to_string()
                            })
                            .collect::<String>()
                    );
                }
            }
            return step;
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let robots = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (ps, vs) = line.split_once(' ').unwrap();
            let (px, py) = ps.strip_prefix("p=").unwrap().split_once(',').unwrap();
            let (vx, vy) = vs.strip_prefix("v=").unwrap().split_once(',').unwrap();
            Robot {
                p: (px.parse().unwrap(), py.parse().unwrap()),
                v: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    (
        solve_a(&robots, 100).to_string(),
        solve_b(&robots).to_string(),
    )
}
