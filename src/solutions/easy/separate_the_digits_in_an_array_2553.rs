/*
 * @lc app=leetcode.cn id=2553 lang=rust
 *
 * [2553] Separate the Digits in an Array
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums
            .iter()
            .map(|x| x.to_string())
            .map(|x| x.chars().collect::<Vec<char>>())
            .fold(
                Vec::with_capacity(nums.len() * 2), 
                |mut acc, x| {
                    acc.extend(x.into_iter().map(|c| c as i32 - '0' as i32)); 
                    acc
                }
            )
    }
}
// @lc code=end

