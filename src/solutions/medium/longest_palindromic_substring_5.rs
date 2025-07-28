/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let cs: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..cs.len() - 1 {
            let (l1, r1) = Self::expand(&cs, i, i);
            let (l2, r2) = Self::expand(&cs, i, i + 1);

            if r1 >= l1 && r1 - l1 + 1 > max_len {
                start = l1;
                max_len = r1 - l1 + 1;
            }
            if r2 >= l2 && r2 - l2 + 1 > max_len {
                start = l2;
                max_len = r2 - l2 + 1;
            }
        }

        cs[start..start + max_len].iter().collect()
    }

    fn expand(cs: &[char], mut l: usize, mut r: usize) -> (usize, usize) {
        while 0 < l && r < cs.len() - 1 && cs[l] == cs[r] {
            l -= 1;
            r += 1;
        }
        if cs[l] != cs[r] {
            l += 1;
            r -= 1;
        }
        return (l, r);
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::medium::longest_palindromic_substring_5::Solution;

    #[test]
    fn test_1() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_2() {
        let s = "ab".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_3() {
        let s = "xxccaba".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "aba");
    }
}
