/*
 * @lc app=leetcode.cn id=2144 lang=rust
 *
 * [2144] Minimum Cost of Buying Candies With Discount
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        let mut sum = cost.iter().sum::<i32>();
        if cost.len() < 3 { return sum; }
        let mut k = cost.len();
        while k >= 3 {
            sum -= cost[k-3];
            k -= 3;
        }
        sum
    }
}
// @lc code=end

