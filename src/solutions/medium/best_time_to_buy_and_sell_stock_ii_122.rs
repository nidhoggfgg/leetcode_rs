/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for x in prices.windows(2) {
            if x[1]>x[0]{result+=x[1]-x[0]; }
        }
        result
    }
}
// @lc code=end

