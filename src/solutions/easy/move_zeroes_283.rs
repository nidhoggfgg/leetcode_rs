/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] Move Zeroes
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut fast=0;
        let len=nums.len();
        for i in 0..len{
            if fast<i{fast=i}
            if nums[i]==0{
                while fast<len{
                    if nums[fast]!=0{nums.swap(i, fast);break}
                    fast+=1;
                }
            }
        }
    }
}
// @lc code=end

