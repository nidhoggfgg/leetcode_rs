/*
 * @lc app=leetcode.cn id=1109 lang=rust
 *
 * [1109] Corporate Flight Bookings
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n=n as usize;
        let mut ans=vec![0;n+1];
        for x in bookings{
            ans[x[0] as usize-1]+=x[2];
            ans[x[1] as usize]-=x[2];
        }
        for i in 1..n{ans[i]+=ans[i-1];}
        ans.pop();
        ans
    }
}
// @lc code=end

