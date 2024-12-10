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

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use crate::common::Solution;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Fragment {
    id: usize,
    start: usize,
    len: usize,
}

impl Ord for Fragment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Fragment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn defragment(
    mut files: Vec<Fragment>,
    mut gaps: BTreeMap<usize, BTreeSet<usize>>,
) -> Vec<Fragment> {
    for file in files.iter_mut().rev() {
        if let Some((len, start)) = gaps
            .range(file.len..)
            .flat_map(|(len, starts)| {
                starts
                    .range(..file.start)
                    .next()
                    .map(move |start| (*len, *start))
            })
            .min_by_key(|(_, start)| *start)
        {
            gaps.get_mut(&len).unwrap().remove(&start);
            file.start = start;
            if len > file.len {
                gaps.entry(len - file.len)
                    .or_default()
                    .insert(start + file.len);
            }
        }
    }
    files
}

fn solve_a(files: &[Fragment]) -> usize {
    let mut checksum = 0;
    let mut front_i = 0;
    let mut back_i = files.len() - 1;
    let mut file_len = files[back_i].len;
    while front_i < back_i {
        checksum += (files[front_i].start..(files[front_i].start + files[front_i].len))
            .sum::<usize>()
            * front_i;

        let mut gap_start = files[front_i].start + files[front_i].len;
        let gap_end = files[front_i + 1].start;
        let mut gap_len = gap_end - gap_start;

        while gap_len > 0 && front_i < back_i {
            let move_len = std::cmp::min(gap_len, file_len);
            checksum += (gap_start..gap_start + move_len).sum::<usize>() * back_i;

            gap_start += move_len;
            gap_len -= move_len;
            file_len -= move_len;
            if file_len == 0 {
                back_i -= 1;
                if back_i == front_i {
                    return checksum;
                }
                file_len = files[back_i].len;
            }
        }
        front_i += 1;
    }
    checksum += (files[back_i].start..files[back_i].start + file_len).sum::<usize>() * back_i;
    checksum
}

fn solve_b(files: Vec<Fragment>, gaps: BTreeMap<usize, BTreeSet<usize>>) -> usize {
    defragment(files, gaps)
        .into_iter()
        .map(|f| (f.start..(f.start + f.len)).sum::<usize>() * f.id)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (_, _, _, files, gaps): (_, _, _, Vec<Fragment>, BTreeSet<(usize, usize)>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(
            (0, 0, true, Vec::new(), BTreeSet::new()),
            |(start, mut next_id, is_file, mut files, mut gaps), len| {
                if len > 0 {
                    if is_file {
                        files.push(Fragment {
                            id: next_id,
                            start,
                            len,
                        });
                        next_id += 1;
                    } else {
                        gaps.insert((start, len));
                    }
                    (start + len, next_id, !is_file, files, gaps)
                } else {
                    (start, next_id, !is_file, files, gaps)
                }
            },
        );

    (
        solve_a(&files).to_string(),
        solve_b(
            files,
            gaps.into_iter()
                .fold(BTreeMap::new(), |mut gaps, (start, len)| {
                    gaps.entry(len).or_default().insert(start);
                    gaps
                }),
        )
        .to_string(),
    )
}
