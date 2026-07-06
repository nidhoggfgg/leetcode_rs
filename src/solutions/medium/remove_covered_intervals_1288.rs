/*
 * @lc app=leetcode.cn id=1288 lang=rust
 *
 * [1288] Remove Covered Intervals
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b|a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));
        let mut ans = 0;
        let mut maxr = 0;
        for x in intervals {
            if x[1]>maxr{maxr=x[1];ans+=1;}
        }
        ans
    }
}
// @lc code=end

