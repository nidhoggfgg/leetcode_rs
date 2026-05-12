/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] Reverse String
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        let mut left = 0;
        let mut right = len-1;
        while left < right {
            (s[left], s[right]) = (s[right], s[left]);
            left += 1;
            right -= 1;
        }
    }
}
// @lc code=end

