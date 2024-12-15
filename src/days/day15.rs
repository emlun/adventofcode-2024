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

#[cfg(feature = "print")]
fn print_state<const WIDE: bool>(
    walls: &[Vec<bool>],
    boxes: &HashSet<(usize, usize)>,
    pos: (usize, usize),
    dir: u8,
) {
    println!(
        "    {}",
        walls[0]
            .iter()
            .enumerate()
            .map(|(i, _)| if i % 10 == 0 {
                (i / 10).to_string()
            } else {
                " ".to_string()
            })
            .collect::<String>()
    );
    println!(
        "    {}",
        walls[0]
            .iter()
            .enumerate()
            .map(|(i, _)| (i % 10).to_string())
            .collect::<String>()
    );
    for (r, row) in walls.iter().enumerate() {
        println!(
            "{:02}  {}",
            r,
            (0..row.len())
                .map(|c| {
                    if walls[r][c] {
                        '#'
                    } else if boxes.contains(&(r, c)) {
                        if WIDE {
                            '['
                        } else {
                            'O'
                        }
                    } else if WIDE && boxes.contains(&(r, c - 1)) {
                        ']'
                    } else if pos == (r, c) {
                        match dir {
                            0 => '^',
                            1 => '>',
                            2 => 'v',
                            3 => '<',
                            _ => unreachable!(),
                        }
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
        #[cfg(feature = "print")]
        print_state::<false>(walls, &boxes, (r, c), *dir);
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

fn collect_moving_boxes(
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
    boxes: &HashSet<(usize, usize)>,
    walls: &[Vec<bool>],
) -> HashSet<(usize, usize)> {
    if !walls[r][c] {
        if dr != 0 {
            if let Some((br, bc)) = boxes
                .get(&(r, c))
                .or_else(|| boxes.get(&(r, c - 1)))
                .copied()
            {
                let rr = br.wrapping_add_signed(dr);
                let cc = bc.wrapping_add_signed(dc);
                let mut moving = collect_moving_boxes(rr, cc, dr, dc, boxes, walls);
                moving.extend(collect_moving_boxes(rr, cc + 1, dr, dc, boxes, walls));
                moving.insert((br, bc));
                moving
            } else {
                HashSet::with_capacity(0)
            }
        } else if dc != 0 {
            let dcc = if dc < 0 { dc } else { 2 * dc };
            if let Some((br, bc)) = boxes
                .get(&(r, c))
                .or_else(|| boxes.get(&(r, c - 1)))
                .copied()
            {
                let rr = br.wrapping_add_signed(dr);
                let cc = bc.wrapping_add_signed(dcc);
                let mut moving = collect_moving_boxes(rr, cc, dr, dc, boxes, walls);
                moving.insert((br, bc));
                moving
            } else {
                HashSet::with_capacity(0)
            }
        } else {
            HashSet::with_capacity(0)
        }
    } else {
        HashSet::with_capacity(0)
    }
}

fn solve_b(
    walls: Vec<Vec<bool>>,
    boxes: HashSet<(usize, usize)>,
    start: (usize, usize),
    moves: &[u8],
) -> usize {
    let (mut r, mut c) = start;
    c *= 2;

    let walls: Vec<Vec<bool>> = walls
        .into_iter()
        .map(|row| row.into_iter().flat_map(|cell| [cell, cell]).collect())
        .collect();
    let mut boxes: HashSet<(usize, usize)> = boxes
        // let mut boxes: HashSet<(usize, usize, bool)> = boxes
        .into_iter()
        // .flat_map(|(r, c)| [(r, c * 2, true), (r, c * 2 + 1, false)])
        .map(|(r, c)| (r, c * 2))
        .collect();

    for dir in moves {
        #[cfg(feature = "print")]
        print_state::<true>(&walls, &boxes, (r, c), *dir);

        let (dr, dc): (isize, isize) = match dir {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        let rr = r.checked_add_signed(dr).unwrap();
        let cc = c.checked_add_signed(dc).unwrap();
        if !walls[rr][cc] {
            let boxes_moving = collect_moving_boxes(rr, cc, dr, dc, &boxes, &walls);
            if boxes_moving.iter().all(|(br, bc)| {
                let brr = br.wrapping_add_signed(dr);
                let bcc = bc.wrapping_add_signed(dc);
                !walls[brr][bcc] && !walls[brr][bcc + 1]
            }) {
                r = rr;
                c = cc;
                for bx in &boxes_moving {
                    boxes.remove(bx);
                }
                boxes.extend(
                    boxes_moving
                        .into_iter()
                        .map(|(br, bc)| (br.wrapping_add_signed(dr), bc.wrapping_add_signed(dc))),
                );
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
        solve_a(&walls, boxes.clone(), start, &moves).to_string(),
        solve_b(walls, boxes, start, &moves).to_string(),
    )
}
