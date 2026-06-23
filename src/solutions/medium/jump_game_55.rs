/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] Jump Game
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 { return true; }
        let mut count = 0;
        let first = nums[0];
        for x in nums.into_iter().rev() {
            if x >= count {
                count = 0;
            }
            count+=1;
        }
        first >= count
    }
}
// @lc code=end

