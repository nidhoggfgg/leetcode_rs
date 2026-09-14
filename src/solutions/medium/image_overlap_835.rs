/*
 * @lc app=leetcode.cn id=835 lang=rust
 *
 * [835] Image Overlap
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut ds = HashMap::new();
        let mut ones1 = Vec::new();
        let mut ones2 = Vec::new();
        for (i, row) in img1.into_iter().enumerate() {
            for (j, x) in row.into_iter().enumerate() {
                if x == 1 {
                    ones1.push((100 * i + j) as i32);
                }
            }
        }
        for (i, row) in img2.into_iter().enumerate() {
            for (j, x) in row.into_iter().enumerate() {
                if x == 1 {
                    ones2.push((100 * i + j) as i32);
                }
            }
        }
        for a in ones1 {
            for b in ones2.iter() {
                ds.entry(a - b).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        *ds.values().max().unwrap_or(&0)
    }
}
// @lc code=end
