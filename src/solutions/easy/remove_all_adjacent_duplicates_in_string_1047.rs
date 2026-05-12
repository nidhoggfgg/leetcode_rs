/*
 * @lc app=leetcode.cn id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        let mut last_char = '\0';
        for c in s.chars() {
            if c == last_char {
                stack.pop();
                last_char = if stack.len() == 0 { '\0' } else { stack[stack.len()-1] };
            } else {
                stack.push(c);
                last_char = c;
            }
        }

        stack.into_iter().collect()
    }
}
// @lc code=end

