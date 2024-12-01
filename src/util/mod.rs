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

#![allow(unused)]

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

pub mod iter;

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    let gcdab = gcd(a, b);
    (a / gcdab) * b
}

/**
 * The sum of the odd positive integers less than or equal to `n`.
 */
pub fn sum_odd<Int>(n: Int) -> Int
where
    Int: From<u8>,
    Int: Copy,
    Int: PartialEq,
    Int: Add<Output = Int>,
    Int: Sub<Output = Int>,
    Int: Mul<Output = Int>,
    Int: Div<Output = Int>,
    Int: Rem<Output = Int>,
{
    let zero = Int::from(0);
    if n == zero {
        zero
    } else {
        let one: Int = Int::from(1);
        let two: Int = Int::from(2);
        let n_plus_one = n + one;
        let n_minus_one = n - one;
        (n * n_plus_one / two + n * (n % two) - ((n_minus_one / two) + (n_minus_one % two))) / two
    }
}

/**
 * The sum of the even positive integers less than or equal to `n`.
 */
pub fn sum_even<Int>(n: Int) -> Int
where
    Int: From<u8>,
    Int: Copy,
    Int: PartialEq,
    Int: Add<Output = Int>,
    Int: Sub<Output = Int>,
    Int: Mul<Output = Int>,
    Int: Div<Output = Int>,
    Int: Rem<Output = Int>,
{
    let one: Int = Int::from(1);
    let two: Int = Int::from(2);
    let n_plus_one = n + one;
    let n_mod_two = n % two;
    (n * n_plus_one / two + (n / two) + n_mod_two - n_plus_one * n_mod_two) / two
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_odd() {
        for n in 0..=1_000 {
            assert_eq!(
                (0..=n).filter(|i| i % 2 == 1).sum::<u32>(),
                super::sum_odd(n),
                "n={n}"
            );
        }
    }

    #[test]
    fn sum_even() {
        for n in 0..=1_000 {
            assert_eq!(
                (0..=n).filter(|i| i % 2 == 0).sum::<u32>(),
                super::sum_even(n),
                "n={n}"
            );
        }
    }

    #[test]
    fn sum_even_and_odd() {
        for n in 0..10_u8 {
            assert_eq!(
                n * (n + 1) / 2,
                super::sum_even(n) + super::sum_odd(n),
                "n={n}"
            );
        }
        for n in 10..100_u16 {
            assert_eq!(
                n * (n + 1) / 2,
                super::sum_even(n) + super::sum_odd(n),
                "n={n}"
            );
        }
        for n in 100..10_000_u32 {
            assert_eq!(
                n * (n + 1) / 2,
                super::sum_even(n) + super::sum_odd(n),
                "n={n}"
            );
        }
        for n in 10_000..=100_000_u64 {
            assert_eq!(
                n * (n + 1) / 2,
                super::sum_even(n) + super::sum_odd(n),
                "n={n}"
            );
        }
    }
}
