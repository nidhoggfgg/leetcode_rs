/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // O(n)
        // let mut result = vec![0; nums.len()];
        // let (mut left, mut right) = (0, nums.len() - 1);
        // let mut t = right;
        // while left <= right {
        //     if nums[left].abs() < nums[right].abs() {
        //         result[t] = nums[right].pow(2);
        //         right -= 1;
        //     } else {
        //         result[t] = nums[left].pow(2);
        //         left += 1;
        //     }
        //     t -= 1;
        // }
        // return result;

        let mut result: Vec<i32> = nums.into_iter().map(|x| x.pow(2)).collect();
        result.sort_unstable();
        return result;
    }
}
// @lc code=end

