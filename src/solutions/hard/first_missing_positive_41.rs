/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] First Missing Positive
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        for i in 0..len as usize {
            while nums[i] > 0 && nums[i] <= len && nums[i] != nums[nums[i] as usize-1] {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            }
        }
        for i in 0..len as usize {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        len as i32 + 1
    }
}
// @lc code=end

