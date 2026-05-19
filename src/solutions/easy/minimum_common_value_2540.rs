/*
 * @lc app=leetcode.cn id=2540 lang=rust
 *
 * [2540] Minimum Common Value
 */

pub struct Solution;

// @lc code=start
use std::cmp::Ordering::*;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let len1 = nums1.len();
        let len2 = nums2.len();
        while i<len1 && j<len2 {
            match nums1[i].cmp(&nums2[j]) {
                Less => i+=1,
                Equal => return nums1[i],
                Greater => j+=1,
            }
        }

        -1
    }
}
// @lc code=end

