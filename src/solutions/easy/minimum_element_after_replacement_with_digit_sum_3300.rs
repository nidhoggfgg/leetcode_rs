/*
 * @lc app=leetcode.cn id=3300 lang=rust
 *
 * [3300] Minimum Element After Replacement With Digit Sum
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        for mut x in nums {
            let mut sum = 0;
            while x > 0 {
                sum+=x%10;
                x/=10;
            }
            if sum < result { result = sum; }
        }
        result
    }
}
// @lc code=end

