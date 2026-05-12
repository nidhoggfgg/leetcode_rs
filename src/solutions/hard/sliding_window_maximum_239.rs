/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */


pub struct Solution;
// @lc code=start
use std::collections::VecDeque;
pub struct Pque {
    que: VecDeque<(usize, i32)>,
}

impl Pque {
    pub fn new() -> Self {
        Self { que: VecDeque::new() }
    }

    pub fn pop(&mut self, index: usize) {
        if let Some((i, _)) = self.que.front() {
            if *i == index {
                self.que.pop_front();
            }
        }
    }

    pub fn push(&mut self, index: usize, value: i32) {
        while let Some((_, v)) = self.que.back() {
            if *v < value {
                self.que.pop_back();
            } else {
                break;
            }
        }
        self.que.push_back((index, value));
    }

    pub fn max(&self) -> i32 {
        let (_, v) = self.que.front().unwrap();
        *v
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k as usize);
        let mut pque = Pque::new();
        for i in 0..k {
            pque.push(i, nums[i]);
        }
        result.push(pque.max());
        for i in k..nums.len() {
            pque.pop(i-k);
            pque.push(i, nums[i]);
            result.push(pque.max());
        }

        result
    }
}
// @lc code=end

