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

use std::collections::HashSet;

use crate::common::Solution;

#[cfg(debug_assertions)]
fn print_state(walls: &[Vec<bool>], boxes: &HashSet<(usize, usize)>, pos: (usize, usize)) {
    for r in 0..walls.len() {
        println!(
            "{:02}  {}",
            r,
            (0..walls[r].len())
                .map(|c| {
                    if walls[r][c] {
                        '#'
                    } else if boxes.contains(&(r, c)) {
                        'O'
                    } else if pos == (r, c) {
                        '@'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
    println!()
}

fn solve_a(
    walls: &[Vec<bool>],
    mut boxes: HashSet<(usize, usize)>,
    start: (usize, usize),
    moves: &[u8],
) -> usize {
    let (mut r, mut c) = start;
    for dir in moves {
        #[cfg(debug_assertions)]
        print_state(walls, &boxes, (r, c));
        let (dr, dc): (isize, isize) = match dir {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        let rr = r.checked_add_signed(dr).unwrap();
        let cc = c.checked_add_signed(dc).unwrap();
        let move_valid = !walls[rr][cc]
            && !std::iter::successors(Some((rr, cc)), |(r, c)| {
                Some((r.checked_add_signed(dr)?, c.checked_add_signed(dc)?))
            })
            .take_while(|(rr, cc)| !walls[*rr][*cc])
            .all(|pos| boxes.contains(&pos));
        if move_valid {
            r = rr;
            c = cc;
            if boxes.contains(&(r, c)) {
                boxes.remove(&(r, c));
                let mut box_r = r.checked_add_signed(dr).unwrap();
                let mut box_c = c.checked_add_signed(dc).unwrap();
                while boxes.contains(&(box_r, box_c)) {
                    box_r = box_r.checked_add_signed(dr).unwrap();
                    box_c = box_c.checked_add_signed(dc).unwrap();
                }
                boxes.insert((box_r, box_c));
            }
        }
    }
    boxes.iter().map(|(r, c)| r * 100 + c).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (walls, boxes, start) = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .enumerate()
        .fold(
            (Vec::new(), HashSet::new(), (0, 0)),
            |(mut walls, boxes, start), (r, line)| {
                let (row, boxes, start) = line.chars().enumerate().fold(
                    (Vec::with_capacity(line.len()), boxes, start),
                    |(mut row, mut boxes, mut start), (c, ch)| {
                        row.push(ch == '#');
                        match ch {
                            'O' => {
                                boxes.insert((r, c));
                            }
                            '@' => start = (r, c),
                            _ => {}
                        };
                        (row, boxes, start)
                    },
                );
                walls.push(row);
                (walls, boxes, start)
            },
        );

    let moves: Vec<u8> = lines
        .iter()
        .skip_while(|line| line.is_empty())
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .map(|ch| match ch {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => unreachable!(),
        })
        .collect();

    (
        solve_a(&walls, boxes, start, &moves).to_string(),
        "".to_string(),
    )
}
