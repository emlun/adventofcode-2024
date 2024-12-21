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

fn type_digit(
    digit: usize,
    current_keypad: &[(isize, isize)],
    code_keypad: &[(isize, isize)],
    prev_digit: usize,
) -> Vec<Vec<usize>> {
    let code_keypad_pos = code_keypad[prev_digit];
    let (x, y) = code_keypad_pos;
    let (tx, ty) = code_keypad[digit];
    let dx = tx - x;
    let dy = ty - y;

    let btn_x = if dx >= 0 { RIGHT } else { LEFT };
    let btn_y = if dy > 0 { DOWN } else { UP };

    let mut seq = Vec::new();

    if code_keypad.contains(&(tx, y)) {
        let mut seq_x_first = Vec::new();
        seq_x_first.resize(dx.abs() as usize, btn_x);
        seq_x_first.resize((dx.abs() + dy.abs()) as usize, btn_y);
        seq_x_first.push(current_keypad.len() - 1);
        seq.push(seq_x_first);
    }
    if code_keypad.contains(&(x, ty)) {
        let mut seq_y_first = Vec::new();
        seq_y_first.resize(dy.abs() as usize, btn_y);
        seq_y_first.resize((dy.abs() + dx.abs()) as usize, btn_x);
        seq_y_first.push(current_keypad.len() - 1);
        if seq.is_empty() || seq[0] != seq_y_first {
            seq.push(seq_y_first);
        }
    }
    seq
}

fn type_code(
    code: &[usize],
    current_keypad: &[(isize, isize)],
    code_keypad: &[(isize, isize)],
    mut prev_digit: usize,
) -> Vec<Vec<usize>> {
    let mut options = type_digit(code[0], current_keypad, code_keypad, prev_digit);
    prev_digit = code[0];

    for digit in &code[1..] {
        let new_options = type_digit(*digit, current_keypad, code_keypad, prev_digit);
        let mut dup_options = options.clone();
        for opt in &mut options {
            opt.extend(&new_options[0]);
        }
        if new_options.len() > 1 {
            for opt in &mut dup_options {
                opt.extend(&new_options[1]);
            }
            options.extend(dup_options);
        }
        prev_digit = *digit;
    }

    options
}

fn solve_a(codes: &[&str]) -> usize {
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

            let lv1_options = type_code(&code, &DIR_KEYPAD, &NUM_KEYPAD, NUM_KEYPAD.len() - 1);
            let lv2_options = lv1_options
                .iter()
                .flat_map(|code| type_code(code, &DIR_KEYPAD, &DIR_KEYPAD, DIR_KEYPAD.len() - 1))
                .collect::<Vec<_>>();
            let lv3_options = lv2_options
                .iter()
                .flat_map(|code| type_code(code, &DIR_KEYPAD, &DIR_KEYPAD, DIR_KEYPAD.len() - 1))
                .collect::<Vec<_>>();

            let shortest_len = lv3_options.iter().map(|opt| opt.len()).min().unwrap();
            shortest_len * num_code
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let codes: Vec<&str> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .collect();

    (solve_a(&codes).to_string(), "".to_string())
}
