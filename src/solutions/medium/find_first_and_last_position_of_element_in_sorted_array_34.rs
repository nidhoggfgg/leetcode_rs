/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let x = Self::binary_search(&nums, target, 0, len);
        if x == -1 { return vec![-1, -1] }
        let mut left = x as usize;
        let mut right = x as usize;
        while left > 0 && nums[left-1] == target { left -=1; }
        while right < len - 1 && nums[right+1] == target { right += 1; }
        vec![left as i32, right as i32]
    }

    // i..j
    fn binary_search(nums: &[i32], target: i32, mut start: usize, mut end: usize) -> i32 {
        while start < end {
            let mid = start + (end - start) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Less => start = mid + 1,
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Greater => end = mid,
            }
        }
        -1
    }
}
// @lc code=end

