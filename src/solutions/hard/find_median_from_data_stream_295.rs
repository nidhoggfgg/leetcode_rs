/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] Find Median from Data Stream
 */

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};
struct MedianFinder {
    large: BinaryHeap<Reverse<i32>>,
    small: BinaryHeap<i32>,
    even: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            large: BinaryHeap::new(),
            small: BinaryHeap::new(),
            even: true,
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        let v = self.small.pop().unwrap();
        self.large.push(Reverse(v));
        if self.small.len() < self.large.len() {
            let Reverse(v) = self.large.pop().unwrap();
            self.small.push(v);
        }
        self.even = !self.even;
    }

    fn find_median(&self) -> f64 {
        if self.even {
            let a = *self.small.peek().unwrap();
            let Reverse(b) = *self.large.peek().unwrap();
            return (a + b) as f64 / 2.0;
        }
        *self.small.peek().unwrap() as f64
    }
}

// @lc code=end
