/*
 * @lc app=leetcode.cn id=303 lang=rust
 *
 * [303] Range Sum Query - Immutable
 */

// @lc code=start
pub struct NumArray{p:Vec<i32>}
impl NumArray {
    pub fn new(mut nums: Vec<i32>) -> Self {
        for i in 1..nums.len(){nums[i]+=nums[i-1];}
        NumArray{p:nums}
    }
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (l,r)=(left as usize,right as usize);
        if left>0{self.p[r]-self.p[l-1]}
        else{self.p[r]}
    }
}
// @lc code=end

