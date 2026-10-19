/*
 * @lc app=leetcode.cn id=2089 lang=rust
 *
 * [2089] Find Target Indices After Sorting Array
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index = 0;
        let mut count = 0;
        for x in nums {
            if x < target {
                index += 1;
            } else if x == target {
                count += 1;
            }
        }

        (index..(index + count)).collect()
    }
}
// @lc code=end
