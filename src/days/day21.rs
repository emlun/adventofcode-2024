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

use crate::{common::Solution, util::iter::WithSliding};

const NUM_KEYPAD: &[(isize, isize)] = &[
    (2, 4),
    (1, 3),
    (2, 3),
    (3, 3),
    (1, 2),
    (2, 2),
    (3, 2),
    (1, 1),
    (2, 1),
    (3, 1),
    (3, 4),
];

const DIR_KEYPAD: &[(isize, isize)] = &[(2, 1), (1, 2), (2, 2), (3, 2), (3, 1)];
const UP: usize = 0;
const LEFT: usize = 1;
const DOWN: usize = 2;
const RIGHT: usize = 3;

type Presses = HashMap<(usize, usize), isize>;

fn merge_with<K, V, F>(mut a: HashMap<K, V>, b: HashMap<K, V>, f: F) -> HashMap<K, V>
where
    K: std::hash::Hash,
    K: std::cmp::Eq,
    V: Default,
    F: Fn(&V, V) -> V,
{
    for (k, vb) in b {
        let va = a.entry(k).or_default();
        let vv = f(va, vb);
        *va = vv;
    }
    a
}

fn expand_presses(
    presses: HashMap<(usize, usize), isize>,
    prev_keypad: &[(isize, isize)],
    next_keypad: &[(isize, isize)],
    memo: &mut HashMap<(usize, usize), Presses>,
    prefer_x: &HashMap<(isize, isize), bool>,
) -> Presses {
    presses
        .iter()
        .map(|(k, v)| (*k, *v))
        .map(|((prev_btn, press_btn), count)| {
            memo.entry((prev_btn, press_btn))
                .or_insert_with(|| {
                    let (x, y) = prev_keypad[prev_btn];
                    let (tx, ty) = prev_keypad[press_btn];
                    let dx = tx - x;
                    let dy = ty - y;

                    let btn_a = next_keypad.len() - 1;
                    let btn_x = if dx >= 0 { RIGHT } else { LEFT };
                    let btn_y = if dy >= 0 { DOWN } else { UP };

                    let x_first = if *prefer_x.get(&(dx, dy)).unwrap_or(&true) {
                        prev_keypad.contains(&(tx, y))
                    } else {
                        !prev_keypad.contains(&(x, ty))
                    };

                    let order = if x_first {
                        [(btn_x, dx), (btn_y, dy), (btn_a, 1)]
                    } else {
                        [(btn_y, dy), (btn_x, dx), (btn_a, 1)]
                    };

                    let (expanded, _) = order.iter().copied().fold(
                        (HashMap::new(), btn_a),
                        |(mut exp, current_btn), (btn, d)| {
                            if d.abs() >= 1 {
                                *exp.entry((current_btn, btn)).or_default() += 1;
                                if d.abs() > 1 {
                                    *exp.entry((btn, btn)).or_default() += d.abs() - 1;
                                }
                                (exp, btn)
                            } else {
                                (exp, current_btn)
                            }
                        },
                    );
                    expanded
                })
                .iter()
                .map(|(btns, c)| (*btns, *c * count))
                .collect()
        })
        .fold(HashMap::new(), |acc, presses| {
            merge_with(acc, presses, |a, b| a + b)
        })
}

fn expand_layers(codes: &[&str], layers: usize, prefer_x: &HashMap<(isize, isize), bool>) -> usize {
    codes
        .iter()
        .map(|code| {
            let num_code: usize = code.strip_suffix("A").unwrap().parse().unwrap();
            let code = code
                .chars()
                .map(|ch| match ch {
                    'A' => 10,
                    ch => ch.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<_>>();

            let mut presses: HashMap<(usize, usize), isize> = [NUM_KEYPAD.len() - 1]
                .iter()
                .chain(code.iter())
                .copied()
                .sliding2()
                .fold(HashMap::new(), |mut presses, (prev_btn, press_btn)| {
                    *presses.entry((prev_btn, press_btn)).or_default() += 1;
                    presses
                });

            presses = expand_presses(
                presses,
                NUM_KEYPAD,
                DIR_KEYPAD,
                &mut HashMap::new(),
                prefer_x,
            );

            let mut memo = HashMap::new();
            for _ in 0..(layers - 1) {
                presses = expand_presses(presses, DIR_KEYPAD, DIR_KEYPAD, &mut memo, prefer_x);
            }
            let l = presses.values().sum::<isize>();
            l as usize * num_code
        })
        .sum()
}

fn solve_ab(codes: &[&str], layers: usize) -> usize {
    let mut prefer_x = HashMap::new();
    let dxys: Vec<(isize, isize)> = (-2..=2)
        .flat_map(|dx| (-3..=3).map(move |dy| (dx, dy)))
        .filter(|(dx, dy)| *dx != 0 && *dy != 0 && (*dx, *dy) != (-2, 3) && (*dx, *dy) != (2, -3))
        .collect();
    let mut best = expand_layers(codes, layers, &prefer_x);

    loop {
        let mut changed = false;
        for dxy in &dxys {
            let pref = *prefer_x.get(dxy).unwrap_or(&true);
            prefer_x.insert(*dxy, !pref);
            let shortest = expand_layers(codes, layers, &prefer_x);
            if shortest < best {
                best = shortest;
                changed = true;
            } else {
                prefer_x.insert(*dxy, pref);
            }
        }
        if !changed {
            break best;
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let codes: Vec<&str> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .collect();

    (
        solve_ab(&codes, 3).to_string(),
        solve_ab(&codes, 26).to_string(),
    )
}
