/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 * [541] Reverse String II
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars = s
            .chars()
            .collect::<Vec<char>>();
        chars.chunks_mut(2 * k)
            .for_each(|x| {
                if x.len() > k {
                    Solution::reverse_k(x, k);
                } else {
                    Solution::reverse_k(x, x.len());
                }
            });
        chars.iter().collect()
    }

    fn reverse_k(s: &mut [char], k: usize) {
        let mut l = 0;
        let mut r = k - 1;
        while l < r {
            (s[l], s[r]) = (s[r], s[l]);
            l += 1; r -= 1;
        }
    }
}
// @lc code=end

