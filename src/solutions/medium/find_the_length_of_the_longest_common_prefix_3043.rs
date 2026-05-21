
/*
 * @lc app=leetcode.cn id=3043 lang=rust
 *
 * [3043] Find the Length of the Longest Common Prefix
 */
pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefixs = HashSet::with_capacity(arr1.len() * 2);

        let mut result = 0;
        for mut x in arr1 {
            while x > 0 {
                prefixs.insert(x);
                x /= 10;
            }
        }

        for mut x in arr2 {
            let mut len = x.ilog10() + 1;
            if len <= result { continue; }
            while x > 0 {
                if prefixs.contains(&x) {
                    result = result.max(len);
                    break;
                }
                x /= 10;
                len -= 1;
            }
        }

        result as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![1, 2, 3];
        let b = vec![4, 4, 4];
        assert_eq!(Solution::longest_common_prefix(a, b), 0);
    }

    #[test]
    fn test_2() {
        let a = vec![13, 27, 45];
        let b = vec![21, 27, 48];
        assert_eq!(Solution::longest_common_prefix(a, b), 2);
    }

    #[test]
    fn test_3() {
        let a = vec![1, 10, 100];
        let b = vec![1000];
        assert_eq!(Solution::longest_common_prefix(a, b), 3);
    }
}
