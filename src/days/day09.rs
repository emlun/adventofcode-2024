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

fn enfragment(
    mut files: BTreeSet<Fragment>,
    mut gaps: BTreeSet<(usize, usize)>,
) -> Result<
    (BTreeSet<Fragment>, BTreeSet<(usize, usize)>),
    (BTreeSet<Fragment>, BTreeSet<(usize, usize)>),
> {
    if let Some(frag) = files.pop_last() {
        if let Some((gap_start, gap_len)) = gaps.pop_first() {
            let len = std::cmp::min(frag.len, gap_len);
            files.insert(Fragment {
                id: frag.id,
                start: gap_start,
                len,
            });

            if frag.len > len {
                files.insert(Fragment {
                    id: frag.id,
                    start: frag.start,
                    len: frag.len - len,
                });
            }

            if gap_len > len {
                gaps.insert((gap_start + len, gap_len - len));
            }
            Ok((files, gaps))
        } else {
            let start = files.last().map(|file| file.start + file.len).unwrap();
            files.insert(Fragment {
                id: frag.id,
                start,
                len: frag.len,
            });
            Err((files, gaps))
        }
    } else {
        Err((files, gaps))
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

fn solve_a(mut files: BTreeSet<Fragment>, mut gaps: BTreeSet<(usize, usize)>) -> usize {
    loop {
        (files, gaps) = match enfragment(files, gaps) {
            Ok(fs2) => fs2,
            Err((files, _)) => {
                break files;
            }
        };
    }
    .into_iter()
    .map(|f| (f.start..(f.start + f.len)).sum::<usize>() * f.id)
    .sum()
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
        solve_a(files.iter().cloned().collect(), gaps.clone()).to_string(),
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
