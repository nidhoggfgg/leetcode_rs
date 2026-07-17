/*
 * @lc app=leetcode.cn id=1094 lang=rust
 *
 * [1094] Car Pooling
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut seats=[0;1001];
        for x in trips{
            seats[x[1] as usize]+=x[0];
            seats[x[2] as usize]-=x[0];
        }
        let mut now=0;
        for x in seats {now+=x;if now>capacity{return false;}}
        true
    }
}
// @lc code=end

