/*
 * @lc app=leetcode.cn id=3658 lang=rust
 *
 * [3658] GCD of Odd and Even Sums
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let n = n as u64;
        let (mut so,mut se)=(n*n,n*(n+1));
        while so!=0{(se,so)=(so,se%so)}
        se as i32
    }
}
// @lc code=end

