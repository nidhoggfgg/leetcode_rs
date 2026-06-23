/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] Maximum Subarray
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = Vec::with_capacity(nums.len());

        let mut sum = 0;
        for x in nums {
            sum += x;
            prefix_sum.push(sum);
        }

        let mut min = i32::MAX;
        let mut result = prefix_sum[0];
        for x in prefix_sum {
            if min < 0 {
                result = result.max(x-min);
            } else {
                result = result.max(x);
            }

            if x < min { min = x; }
        }

        result
    }
}
// @lc code=end

