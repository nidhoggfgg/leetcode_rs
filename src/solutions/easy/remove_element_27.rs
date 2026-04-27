/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] Remove Element
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut last = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[last] = nums[i];
                last += 1;
            }
        }
        return last as i32;
    }
}
// @lc code=end

