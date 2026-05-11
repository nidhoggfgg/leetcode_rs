/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] Happy Number
 */


pub struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();
        while !seen.contains(&n) {
            if n == 1 { return true; }
            seen.insert(n);
            let mut v = 0;
            while n > 0 {
                v += (n % 10).pow(2);
                n /= 10;
            }
            n = v;
        }
        false
    }
}
// @lc code=end

