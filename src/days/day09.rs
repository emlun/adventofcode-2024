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

use crate::{common::Solution, util::iter::WithSliding};

#[derive(Debug)]
struct Fragment {
    id: usize,
    start: usize,
    end: usize,
}

fn enfragment(mut fs: Vec<Fragment>) -> Result<Vec<Fragment>, Vec<Fragment>> {
    if let Some(mut frag) = fs.pop() {
        if let Some((i, start, end)) = fs
            .iter()
            .enumerate()
            .sliding2()
            .map(|((_, fa), (i, fb))| (i, fa.end, fb.start))
            .find(|(_, start, end)| start < end)
        {
            let len = core::cmp::min(end - start, frag.end - frag.start);
            fs.insert(
                i,
                Fragment {
                    id: frag.id,
                    start,
                    end: start + len,
                },
            );
            frag.end -= len;
            if frag.end > frag.start {
                fs.push(frag);
            }
            Ok(fs)
        } else {
            if let Some(prev) = fs.last() {
                let di = frag.start - prev.end;
                frag.start -= di;
                frag.end -= di;
            }
            fs.push(frag);
            Err(fs)
        }
    } else {
        Err(fs)
    }
}

fn solve_a(fs: Vec<Fragment>) -> usize {
    let mut fs = fs;
    let fs_defrag = loop {
        match enfragment(fs) {
            Ok(fs2) => {
                fs = fs2;
            }
            Err(fs2) => {
                break fs2;
            }
        }
    };

    fs_defrag
        .into_iter()
        .map(|f| (f.start..f.end).sum::<usize>() * f.id)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (_, _, _, fs): (_, _, _, Vec<Fragment>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .fold(
            (0, 0, true, Vec::new()),
            |(start, mut next_id, is_file, mut fs), ch| {
                let end = start + ch;
                if is_file {
                    fs.push(Fragment {
                        id: next_id,
                        start,
                        end,
                    });
                    next_id += 1;
                }
                (end, next_id, !is_file, fs)
            },
        );

    (solve_a(fs).to_string(), "".to_string())
}
