/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = l+(r-l)/2;
            // l..m..|..r r<l<m
            // l..|..m..r m<r<l
            // l..m..r| l<m<r
            match nums[m].cmp(&target) {
                std::cmp::Ordering::Less=>
                    if nums[l]<nums[m]||nums[r-1]>=target{l=m+1}
                    else{r=m}
                std::cmp::Ordering::Equal=>return m as i32,
                std::cmp::Ordering::Greater=>
                    if nums[r-1]>nums[m]||nums[l]<=target{r=m}
                    else{l=m+1}
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
