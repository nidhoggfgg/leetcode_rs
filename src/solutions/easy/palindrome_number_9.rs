/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] Palindrome Number
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let cs: Vec<_> = x.to_string().chars().collect();
        let mut l = 0;
        let mut r = cs.len() - 1;
        loop {
            if r <= l {
                break;
            }

            if cs[l] != cs[r] {
                return false;
            }

            l += 1;
            r -= 1;
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::easy::palindrome_number_9::Solution;

    #[test]
    fn test_1() {
        let x = 10;
        assert_eq!(Solution::is_palindrome(x), false);
    }
}
