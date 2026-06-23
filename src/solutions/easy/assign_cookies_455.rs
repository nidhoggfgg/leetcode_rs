/*
 * @lc app=leetcode.cn id=455 lang=rust
 *
 * [455] Assign Cookies
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut i = 0;
        let len = s.len();
        let mut result = 0;
        for x in g {
            while i<len && s[i]<x { i+=1; }
            if i >= len { break; }
            result+=1;
            i+=1;
        }
        result
    }
}
// @lc code=end

