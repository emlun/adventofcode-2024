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
    recursion_limit: usize,
    memo: &mut HashMap<(usize, usize), Presses>,
    prefer_x: &HashMap<(isize, isize), bool>,
) -> Presses {
    let expanded = presses
        .iter()
        .map(|(k, v)| (*k, *v))
        .map(|((prev_btn, press_btn), count)| {
            let expanded =
                if prev_keypad == next_keypad && memo.contains_key(&(prev_btn, press_btn)) {
                    memo[&(prev_btn, press_btn)].clone()
                } else {
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

                    let expanded = if x_first {
                        let mut current_btn = btn_a;
                        let mut x_first = HashMap::new();
                        if dx.abs() >= 1 {
                            *x_first.entry((current_btn, btn_x)).or_default() += 1;
                            *x_first.entry((btn_x, btn_x)).or_default() += dx.abs() - 1;
                            current_btn = btn_x;
                        }
                        if dy.abs() >= 1 {
                            *x_first.entry((current_btn, btn_y)).or_default() += 1;
                            *x_first.entry((btn_y, btn_y)).or_default() += dy.abs() - 1;
                            current_btn = btn_y;
                        }
                        *x_first.entry((current_btn, btn_a)).or_default() += 1;
                        x_first
                    } else {
                        let mut current_btn = btn_a;
                        let mut y_first = HashMap::new();
                        if dy.abs() >= 1 {
                            *y_first.entry((current_btn, btn_y)).or_default() += 1;
                            *y_first.entry((btn_y, btn_y)).or_default() += dy.abs() - 1;
                            current_btn = btn_y;
                        }
                        if dx.abs() >= 1 {
                            *y_first.entry((current_btn, btn_x)).or_default() += 1;
                            *y_first.entry((btn_x, btn_x)).or_default() += dx.abs() - 1;
                            current_btn = btn_x;
                        }
                        *y_first.entry((current_btn, btn_a)).or_default() += 1;
                        y_first
                    };

                    if recursion_limit > 1
                        && !memo.contains_key(&(prev_btn, press_btn))
                        && prev_keypad == next_keypad
                    {
                        memo.insert((prev_btn, press_btn), expanded.clone());
                    }

                    expanded
                };

            expanded
                .into_iter()
                .map(|(btns, c)| (btns, c * count))
                .collect()
        })
        .fold(HashMap::new(), |acc, presses| {
            merge_with(acc, presses, |a, b| a + b)
        });
    expanded
}

fn solve_ab2(codes: &[&str], layers: usize, prefer_x: &HashMap<(isize, isize), bool>) -> usize {
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
                layers,
                &mut HashMap::new(),
                prefer_x,
            );

            let mut memo = HashMap::new();
            for i in 0..(layers - 1) {
                presses = expand_presses(
                    presses,
                    DIR_KEYPAD,
                    DIR_KEYPAD,
                    layers - i,
                    &mut memo,
                    prefer_x,
                );
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
    let mut best = solve_ab2(codes, layers, &prefer_x);

    loop {
        let mut changed = false;
        for dxy in &dxys {
            let pref = *prefer_x.get(dxy).unwrap_or(&true);
            prefer_x.insert(*dxy, !pref);
            let shortest = solve_ab2(codes, layers, &prefer_x);
            if shortest < best {
                best = shortest;
                changed = true;
            } else {
                prefer_x.insert(*dxy, pref);
            }
        }
        if !changed {
            break;
        }
    }

    best
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
