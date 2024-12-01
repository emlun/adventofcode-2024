use crate::{common::Solution, util::iter::Countable};

fn solve_a(left: &[i32], right: &[i32]) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn solve_b(left: &[i32], right: &[i32]) -> i32 {
    let r_counts = right.iter().counts();
    left.iter()
        .map(|l| l * (*r_counts.get(l).unwrap_or(&0) as i32))
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (mut left, mut right) = lines.iter().filter(|line| !line.is_empty()).fold(
        (
            Vec::with_capacity(lines.len()),
            Vec::with_capacity(lines.len()),
        ),
        |(mut left, mut right), line| {
            let mut it = line.split_whitespace();
            left.push(it.next().unwrap().parse().unwrap());
            right.push(it.next().unwrap().parse().unwrap());
            (left, right)
        },
    );

    left.sort();
    right.sort();

    (
        solve_a(&left, &right).to_string(),
        solve_b(&left, &right).to_string(),
    )
}
