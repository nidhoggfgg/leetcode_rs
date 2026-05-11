/*
 * @lc app=leetcode.cn id=454 lang=rust
 *
 * [454] 4Sum II
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut counts = HashMap::new();
        for x in &nums1 {
            for y in &nums2 {
                counts.entry(x + y).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        for x in &nums3 {
            for y in &nums4 {
                if let Some(v) = counts.get(&(-x-y)) {
                    result += v;
                }
            }
        }
        result
    }
}
// @lc code=end

