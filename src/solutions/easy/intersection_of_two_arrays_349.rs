/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */

pub struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums: HashSet<i32> = nums1.into_iter().collect();
        let mut result = HashSet::new();
        nums2.into_iter().filter(|x| nums.contains(x)).for_each(|x| {result.insert(x);});
        result.into_iter().collect()
    }
}
// @lc code=end

