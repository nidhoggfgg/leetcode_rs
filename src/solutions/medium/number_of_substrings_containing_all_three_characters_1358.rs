/*
 * @lc app=leetcode.cn id=1358 lang=rust
 *
 * [1358] Number of Substrings Containing All Three Characters
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut last= [-1_i32; 3];
        for (i, c) in  s.bytes().enumerate() {
            last[(c-b'a')as usize]=i as i32;
            ans += last[0].min(last[1]).min(last[2])+1;
        }
        ans as i32
    }
}
// @lc code=end

