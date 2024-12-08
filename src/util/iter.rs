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
use std::ops::MulAssign;

pub struct Sliding2<I, T> {
    buffer: Option<T>,
    iter: I,
}

impl<I, T> Iterator for Sliding2<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.buffer.and_then(|buffered| {
            self.iter.next().map(|next| {
                self.buffer = Some(next);
                (buffered, next)
            })
        })
    }
}

pub trait WithSliding
where
    Self: Iterator,
    Self: Sized,
{
    fn sliding2(mut self) -> Sliding2<Self, Self::Item> {
        Sliding2 {
            buffer: self.next(),
            iter: self,
        }
    }
}

impl<I> WithSliding for I where I: Iterator {}

pub struct Pairs<'a, T> {
    seq: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for Pairs<'a, T>
where
    T: 'a,
{
    type Item = (&'a T, &'a T);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let a = self.seq.get(self.i)?;
        let b = self.seq.get(self.j)?;
        self.j = (self.j + 1) % self.seq.len();
        if self.j == 0 {
            self.i += 1;
            self.j = self.i + 1;
        }
        Some((a, b))
    }
}

pub trait WithPairs<T>
where
    Self: AsRef<[T]>,
{
    fn pairs(&self) -> Pairs<T> {
        Pairs {
            seq: self.as_ref(),
            i: 0,
            j: 1,
        }
    }
}

impl<I, T> WithPairs<T> for I where I: AsRef<[T]> {}

pub trait LazyProduct<Int>
where
    Self: Iterator<Item = Int>,
    Self: Sized,
    Int: From<u8>,
    Int: PartialEq<Int>,
    Int: MulAssign<Int>,
{
    fn lazy_product(self) -> Int {
        let mut product = Int::from(1_u8);
        let zero: Int = Int::from(0_u8);
        for i in self {
            if i == zero {
                return zero;
            } else {
                product *= i;
            }
        }
        product
    }
}

impl<I, Int> LazyProduct<Int> for I
where
    I: Iterator<Item = Int>,
    Int: From<u8>,
    Int: PartialEq<Int>,
    Int: MulAssign<Int>,
{
}

pub trait Countable<A> {
    fn counts_into(self, init: HashMap<A, usize>) -> HashMap<A, usize>;
    fn counts(self) -> HashMap<A, usize>
    where
        Self: Sized,
    {
        self.counts_into(HashMap::new())
    }
}

impl<A, I> Countable<A> for I
where
    A: Eq,
    A: std::hash::Hash,
    I: Iterator<Item = A>,
{
    fn counts_into(self, init: HashMap<A, usize>) -> HashMap<A, usize> {
        self.fold(init, |mut result, item| {
            result.entry(item).and_modify(|c| *c += 1).or_insert(1);
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Countable;
    use std::collections::HashMap;

    #[test]
    fn count_empty_is_empty() {
        assert_eq!(Vec::<i32>::new().into_iter().counts(), HashMap::new());
    }

    #[test]
    fn count_one_is_one() {
        assert_eq!(
            vec![0].into_iter().counts(),
            vec![(0, 1)].into_iter().collect()
        );
    }

    #[test]
    fn count_many_is_different() {
        assert_eq!(
            vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 6]
                .into_iter()
                .counts(),
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1), (6, 1)]
                .into_iter()
                .collect()
        );
    }
}
