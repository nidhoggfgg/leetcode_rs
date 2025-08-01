/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let (mut i, mut v) = (0, nums[0]);
        for j in 1..nums.len() {
            if nums[j] != v {
                i += 1;
                nums[i] = nums[j];
                v = nums[i]
            }
        }
        i as i32 + 1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3];
        let result = 3;
        assert_eq!(Solution::remove_duplicates(&mut nums), result);
        assert_eq!(nums[0..result as usize], [1, 2, 3]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), result);
        assert_eq!(nums[0..result as usize], [0, 1, 2, 3, 4]);
    }
}
