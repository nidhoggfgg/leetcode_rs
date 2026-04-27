/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] Binary Search
 */

pub struct Solution;

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32;
        while left < right {
            let middle = left + (right - left) / 2;
            match nums[middle as usize].cmp(&target) {
                Ordering::Greater => right = middle,
                Ordering::Equal => return middle as i32,
                Ordering::Less => left = middle + 1,
            }
        }

        return -1;
    }
}
// @lc code=end

