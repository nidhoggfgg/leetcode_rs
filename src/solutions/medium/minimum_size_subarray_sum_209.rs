/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        let mut result = usize::MAX;
        for i in 0..nums.len() {
            sum += nums[i];
            while sum >= target {
                result = result.min(i - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if result == usize::MAX {
            return 0;
        } else {
            return result as i32;
        }
    }
}
// @lc code=end

