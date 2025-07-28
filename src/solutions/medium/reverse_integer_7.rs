/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] Reverse Integer
 */

pub struct Solution {}

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let s = x.to_string();
        let mut zeros = 0;
        let mut result = VecDeque::new();

        if x < 0 {
            for c in s.chars().skip(1) {
                if c == '0' {
                    zeros += 1;
                } else {
                    zeros = 0;
                }
                result.push_front(c);
            }
        } else {
            for c in s.chars() {
                if c == '0' {
                    zeros += 1;
                } else {
                    zeros = 0;
                }
                result.push_front(c);
            }
        }

        let s: String = result.iter().skip(zeros).collect();
        let r = s.parse().unwrap_or(0);
        if x < 0 { -r } else { r }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::medium::reverse_integer_7::Solution;

    #[test]
    fn test_1() {
        let x = -123;
        assert_eq!(Solution::reverse(x), -321);
    }

    #[test]
    fn test_2() {
        let x = -0;
        assert_eq!(Solution::reverse(x), -0);
    }

    #[test]
    fn test_3() {
        let x = -1;
        assert_eq!(Solution::reverse(x), -1);
    }

    #[test]
    fn test_4() {
        let x = 400;
        assert_eq!(Solution::reverse(x), 4);
    }
}
