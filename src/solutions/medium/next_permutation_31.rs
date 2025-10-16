/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] Next Permutation
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                for j in (i..nums.len()).rev() {
                    if nums[j] > nums[i - 1] {
                        nums.swap(i - 1, j);
                        nums[i..].reverse();
                        return;
                    }
                }
            }
        }
        nums.reverse();
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 5, 1]);
    }
}
