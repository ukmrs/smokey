// This module provides Randorst struct for generating
// sorted random numbers in constant space
// used to smoothly extract words from large file in random fashion
//
// Randorst is my rust implementation of the algorithm
// proposed in the following paper:
// Bentley, Jon & Saxe, James. (1980). Generating Sorted Lists of Random Numbers.
// ACM Trans. Math. Softw.. 6. 359-364. 10.1145/355900.355907.

use fastrand::Rng;
use std::ops::{Range, RangeInclusive};

// there is also std::ops::RangeBounds;
pub trait StartEndRange {
    fn get_bounds(self) -> (usize, usize);
}

impl StartEndRange for Range<usize> {
    fn get_bounds(self) -> (usize, usize) {
        (self.start, self.end - 1)
    }
}

impl StartEndRange for RangeInclusive<usize> {
    fn get_bounds(self) -> (usize, usize) {
        self.into_inner()
    }
}

/// Struct for generating sorted random numbers
/// ```ignore
/// // generate 100 sorted random numbers from 0 to 255
/// use smokey::utils::randorst::Randorst;
/// Randorst::gen(100, 0..256);
/// Randorst::gen(100, 0..=255);
/// ```
// TODO shouldn't I use f64?
pub struct Randorst {
    n: usize,
    min: usize,
    curmax: f32,
    extent: f32,
    rng: Rng,
}

impl Iterator for Randorst {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n <= 1 {
            return None;
        }

        self.n -= 1;
        self.curmax *= self.rng.f32().powf(1.0 / self.n as f32);
        // self.curmax instead of (1. - self.curmax) for descending order
        Some(self.min + ((1. - self.curmax) * self.extent) as usize)
    }
}

impl Randorst {
    /// Returns a generator of sorted random numbers
    /// # Panics
    ///
    /// Panics if range is invalid e.g. 0..0
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use smokey::utils::randorst::Randorst;
    /// let mut last: usize = 0;
    /// let (a, b) = (0, 256);
    /// for i in Randorst::gen(100, a..b) {
    ///     assert!(last <= i);
    ///     assert!(i >= a && i < b);
    ///     last = i;
    /// }
    /// ```
    ///
    pub fn gen<R>(n: usize, range: R) -> Self
    where
        R: StartEndRange,
    {
        let (start, end) = range.get_bounds();
        Self {
            min: start,
            n: n + 1,
            extent: ((end + 1) - start) as f32,
            rng: Rng::new(),
            curmax: 1.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn randorst_test_helper(n: usize, a: usize, b: usize) {
        let mut last_in: usize = 0;
        let mut last_ex: usize = 0;
        let exclusive = Randorst::gen(n, a..b);
        let inclusive = Randorst::gen(n, a..=b);
        for (eni, ini) in exclusive.zip(inclusive) {
            // last item cannot be larger than the next
            assert!(last_ex <= eni && last_in <= ini);
            // exclusive bound testing
            assert!(eni >= a && eni < b);
            // inclusive bound testing
            assert!(ini >= a && ini <= b);
            last_in = ini;
            last_ex = eni;
        }
    }

    #[test]
    fn test_randorst() {
        randorst_test_helper(100, 10, 1000);
        randorst_test_helper(100, 0, 1);
        randorst_test_helper(999, 0, 78);
        randorst_test_helper(1_000, 2121, 100_000_000);
    }
}
