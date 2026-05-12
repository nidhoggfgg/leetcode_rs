/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 4Sum
 */

pub struct Solution;

// @lc code=start
use std::cmp::Ordering::*;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = nums.len();
        if len < 4 { return result; }
        nums.sort_unstable();
        for left in 0..len-3 {
            if left > 0 && nums[left] == nums[left-1] { continue; }
            let t = target as i64 - nums[left] as i64;
            let r = Solution::three_sum(&nums, left+1, len, t);
            if r.len() > 0 { r.into_iter().for_each(|mut x| {x.push(nums[left]); result.push(x);});}
        }

        result
    }

    // i..j
    pub fn three_sum(nums: &[i32], i: usize, j: usize, target: i64) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for left in i..j-2 {
            if left > i && nums[left] == nums[left-1] { continue; }
            let mut mid = left + 1;
            let mut right = j-1;
            while mid < right {
                let v = nums[left] as i64 + nums[mid] as i64 + nums[right] as i64;
                match v.cmp(&target) {
                    Less => mid += 1,
                    Equal => {
                        result.push(vec![nums[left], nums[mid], nums[right]]);
                        while mid < right && nums[mid] == nums[mid+1] { mid += 1 }
                        while right > mid && nums[right] == nums[right-1] { right -= 1 }
                        mid += 1;
                        right -= 1;
                    }
                    Greater => right -= 1,
                }
            }
        }
        result
    }
}
// @lc code=end
