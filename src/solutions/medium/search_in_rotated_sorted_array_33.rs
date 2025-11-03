/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if target == nums[mid] {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
