use crate::common::Solution;

fn solve_a(lines: &[String]) -> u32 {
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

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r): (i32, i32)| l.abs_diff(r))
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
