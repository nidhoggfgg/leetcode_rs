
/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] Min Cost Climbing Stairs
 */
pub struct Solution;

// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut last = 0; let mut now = 0;
        for x in cost {
            last = last.min(now)+x;
            swap(&mut last, &mut now);
        }
        last.min(now)
    }
}
// @lc code=end

