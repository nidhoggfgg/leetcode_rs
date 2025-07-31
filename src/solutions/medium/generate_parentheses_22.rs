/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

use super::Solution;

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return Vec::new();
        }
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();

        queue.push_back(("(".to_string(), 1));
        let total = 2 * n;
        for i in 1..total {
            let len = queue.len();
            for _ in 0..len {
                let (mut s, c) = queue.pop_front().unwrap();
                if c < total - i {
                    let mut s1 = s.clone();
                    s1.push('(');
                    queue.push_back((s1, c + 1));
                }
                if c > 0 {
                    s.push(')');
                    queue.push_back((s, c - 1));
                }
            }
        }

        queue.into_iter().map(|x| x.0).collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let n = 1;
        assert_eq!(Solution::generate_parenthesis(n), vec!["()"])
    }

    #[test]
    fn test_2() {
        let n = 3;
        assert_eq!(
            Solution::generate_parenthesis(n),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        )
    }
}
