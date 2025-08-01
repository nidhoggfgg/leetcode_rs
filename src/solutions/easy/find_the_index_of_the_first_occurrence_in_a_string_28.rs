/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_cs: Vec<_> = haystack.chars().collect();
        let needle_cs: Vec<_> = needle.chars().collect();
        let n_start = needle_cs[0];
        let mut i = 0;
        while i < haystack_cs.len() {
            if haystack_cs[i] == n_start {
                let mut j = 1;
                let mut first = true;
                let mut k = 1;
                while j < needle_cs.len() && i + j < haystack_cs.len() {
                    if haystack_cs[i + j] != needle_cs[j] {
                        break;
                    }
                    if first && haystack_cs[i + j] == n_start {
                        k = j;
                        first = false;
                    }
                    j += 1;
                }
                if j == needle_cs.len() {
                    return i as i32;
                } else {
                    i += k;
                }
                continue;
            }
            i += 1;
        }

        -1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 0);
    }

    #[test]
    fn test_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        assert_eq!(Solution::str_str(haystack, needle), -1);
    }

    #[test]
    fn test_3() {
        let haystack = "mississippi".to_string();
        let needle = "issip".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 4);
    }
}
