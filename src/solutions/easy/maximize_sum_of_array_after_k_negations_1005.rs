/*
 * @lc app=leetcode.cn id=1005 lang=rust
 *
 * [1005] Maximize Sum Of Array After K Negations
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut k = k as usize;
        let len = nums.len();
        let mut i = 0;
        while i < len {
            if k > 0 && nums[i] < 0 {
                nums[i] = -nums[i];
                k -= 1;
            } else {
                break;
            }
            i+=1;
        }
        if k%2==1 {
            if i==len||(i>0&&nums[i-1]<nums[i]){nums[i-1]=-nums[i-1];}
            else {nums[i]=-nums[i];}
        }
        nums.iter().sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-4, -6, 9, -2, 2];
        let k = 4;
        assert_eq!(Solution::largest_sum_after_k_negations(nums, k), 19);
    }
}