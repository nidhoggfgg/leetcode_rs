
/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] Two Sum
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_map = HashMap::new();
        let mut result = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            let r = target - n;
            if let Some(j) = index_map.get(&r) {
                result.push(*j as i32);
                result.push(i as i32);
                break;
            }
            index_map.insert(*n, i);
        }
        result
    }
}
// @lc code=end

