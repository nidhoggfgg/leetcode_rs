/*
 * @lc app=leetcode.cn id=376 lang=rust
 *
 * [376] Wiggle Subsequence
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut result = 0;
        let len = nums.len();
        for i in 1..len {
            if diff >= 0 && nums[i] < nums[i-1] {
                result+=1;
                diff = -1;
            }
            if diff <= 0 && nums[i] > nums[i-1] {
                result+=1;
                diff = 1;
            }
        }
        result+1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 3, 3, 2, 5];
        assert_eq!(Solution::wiggle_max_length(nums), 3);
    }
}
