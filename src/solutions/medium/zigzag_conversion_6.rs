/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let num_rows = num_rows as usize;
        let cs: Vec<_> = s.chars().collect();
        let mut result = Vec::new();
        let chunk_size = (num_rows * 2 - 2) as usize;
        let chunk_num = cs.len() / chunk_size;
        let chunk = chunk_size * chunk_num;
        for j in 0..chunk_num {
            result.push(cs[j * chunk_size]);
        }
        if chunk < cs.len() {
            result.push(cs[chunk]);
        }

        if num_rows > 2 {
            for i in 1..(num_rows - 1) {
                for j in 0..chunk_num {
                    result.push(cs[j * chunk_size + i]);
                    result.push(cs[j * chunk_size + chunk_size - i]);
                }
                if chunk + i < cs.len() {
                    result.push(cs[chunk + i]);
                }
                if chunk + chunk_size - i < cs.len() {
                    result.push(cs[chunk + chunk_size - i]);
                }
            }
        }
        for j in 0..chunk_num {
            result.push(cs[j * chunk_size + num_rows - 1]);
        }
        if chunk + num_rows - 1 < cs.len() {
            result.push(cs[chunk + num_rows - 1]);
        }

        result.iter().collect()
    }
}
// @lc code=end
