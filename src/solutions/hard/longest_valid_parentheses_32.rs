/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] Longest Valid Parentheses
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max_len = 0;

        for (i, c) in s.char_indices() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max_len = max_len.max(i as i32 - stack.last().unwrap());
                }
            }
        }
        max_len
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::Solution;

    #[test]
    fn test_1() {
        let s = "(()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_2() {
        let s = "())".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_3() {
        let s = "()()(".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }

    #[test]
    fn test_4() {
        let s = ")()())()()(".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }

    #[test]
    fn test_5() {
        let s = "()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_6() {
        let s = "(()(((()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_7() {
        let s = "()(())".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 6);
    }

    #[test]
    fn test_8() {
        let s = "(()()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }

    #[test]
    fn test_9() {
        let s = "(()())".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 6);
    }
}
