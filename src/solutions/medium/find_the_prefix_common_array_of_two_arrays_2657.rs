/*
 * @lc app=leetcode.cn id=2657 lang=rust
 *
 * [2657] Find the Prefix Common Array of Two Arrays
 */


pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut sa = HashSet::new();
        let mut sb = HashSet::new();
        let len = a.len();
        let mut result = Vec::with_capacity(len);
        let mut now = 0;
        for i in 0..len {
            if a[i] == b[i] {
                now += 1;
                result.push(now);
                continue;
            }
            if sa.contains(&b[i]) { now += 1; }
            if sb.contains(&a[i]) { now += 1; }
            sa.insert(a[i]); sb.insert(b[i]);
            result.push(now);
        }
        result
    }
}
// @lc code=end

