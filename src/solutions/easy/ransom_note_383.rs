/*
 * @lc app=leetcode.cn id=383 lang=rust
 *
 * [383] Ransom Note
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut chars = [0; 26];
        let a = 'a' as usize;
        magazine.chars().for_each(|c| chars[c as usize - a] += 1);
        for c in ransom_note.chars() {
            if chars[c as usize - a] <= 0 {
                return false;
            }
            chars[c as usize - a] -= 1;
        }

        true
    }
}
// @lc code=end

