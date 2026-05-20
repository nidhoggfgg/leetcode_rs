/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] Search Insert Position
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut l = 0;
        let mut r = len;
        if target < nums[0] { return 0; }
        if target > nums[len-1] { return len as i32; }
        let mut mid = 0;
        while l<r {
            mid = l+(r-l)/2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Less => l = mid+1,
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Greater => r = mid,
            }
        }
        if nums[mid] >= target { mid as i32 }
        else { mid as i32 + 1 }
    }
}
// @lc code=end

