/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] Unique Paths
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans = 1_u64;
        let k=m.min(n)-1;let n=m+n-2;
        for i in 1..=k{ans=ans*(n-k+i) as u64/i as u64;}
        ans as i32
    }
}
// @lc code=end

