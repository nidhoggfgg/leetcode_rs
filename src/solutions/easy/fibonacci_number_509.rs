
/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] Fibonacci Number
 */
pub struct Solution;

// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        let mut now = 1;
        let mut last = 0;
        while n > 0 {last=now+last;swap(&mut last, &mut now);n-=1;}
        last
    }
}
// @lc code=end

