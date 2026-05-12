
/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */
pub struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut counts = HashMap::with_capacity(nums.len() / 2);
        for num in nums {
            counts.entry(num).and_modify(|x| *x+=1).or_insert(1);
        }
        let len = counts.len();
        let mut counts: Vec<(i32, i32)> = counts.into_iter().collect();
        let (_, m, result) = counts.select_nth_unstable_by_key(len-k, |(_, v)| *v);
        let mut result: Vec<i32> = result.iter().map(|(k, _)| *k).collect();
        result.push(m.0);
        result
    }
}
// @lc code=end

