/*
 * @lc app=leetcode.cn id=452 lang=rust
 *
 * [452] Minimum Number of Arrows to Burst Balloons
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        points.sort_unstable_by_key(|x| x[1]);
        let mut right = points[0][1];
        for x in points {
            if x[0]>right{right=x[1];ans+=1;}
        }
        ans+1
    }
}
// @lc code=end

