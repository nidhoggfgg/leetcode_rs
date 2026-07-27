/*
 * @lc app=leetcode.cn id=1464 lang=rust
 *
 * [1464] Maximum Product of Two Elements in an Array
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let(mut a,mut b)=(i32::MIN,i32::MIN);
        for x in nums{if x>a{b=a;a=x;continue;}if x>b{b=x}}
        (a-1)*(b-1)
    }
}
// @lc code=end

