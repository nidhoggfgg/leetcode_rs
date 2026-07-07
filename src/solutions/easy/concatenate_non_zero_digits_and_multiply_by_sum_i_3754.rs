/*
 * @lc app=leetcode.cn id=3754 lang=rust
 *
 * [3754] Concatenate Non-Zero Digits and Multiply by Sum I
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut n=n as i64;
        let (mut ans, mut sum, mut f) = (0, 0, 1);
        while n>0{
            let k = n%10;
            if k==0{n/=10;continue}
            sum+=k;ans+=k*f;n/=10;f*=10;
        }
        ans*sum
    }
}
// @lc code=end

