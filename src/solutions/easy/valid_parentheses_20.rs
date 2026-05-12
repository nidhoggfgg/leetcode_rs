/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let map_to_bool = |c| match c {
            '(' | ')' => Some(false),
            '[' | ']' => Some(true),
            '{' | '}' => None,
            _ => unreachable!()
        };
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(map_to_bool(c)),
                ')' | ']' | '}' => {
                    if let Some(v) = stack.pop() {
                        if v != map_to_bool(c) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => unreachable!()
            }
        }

        stack.len() == 0
    }
}
// @lc code=end

