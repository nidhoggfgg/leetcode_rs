
/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] Climbing Stairs
 */
pub struct Solution;

// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut now = 1; let mut last = 1;
        for _ in 0..n {last=now+last;swap(&mut last, &mut now);}
        last   
    }
}
// @lc code=end

