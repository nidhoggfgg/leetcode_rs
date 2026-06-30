
/*
 * @lc app=leetcode.cn id=406 lang=rust
 *
 * [406] Queue Reconstruction by Height
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b|b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        let mut ans = Vec::with_capacity(people.len());
        for x in people {ans.insert(x[1] as usize, x);}
        ans
    }
}
// @lc code=end

