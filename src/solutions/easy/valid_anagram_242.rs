/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] Valid Anagram
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut counts_s = [0; 26];
        let mut counts_t = [0; 26];
        for (x,y) in s.chars().zip(t.chars()) {
            counts_s[x as usize - 97] += 1;
            counts_t[y as usize - 97] += 1;
        }

        for (x, y) in counts_s.iter().zip(counts_t.iter()) {
            if x != y {
                return false;
            }
        }
        true
    }
}
// @lc code=end

