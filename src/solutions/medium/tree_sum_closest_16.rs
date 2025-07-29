/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut result = nums[0] + nums[1] + nums[2];
        let mut last = nums[nums.len() - 1];
        for l in 0..nums.len() - 2 {
            if last == nums[l] {
                continue;
            }
            last = nums[l];
            let (mut m, mut r) = (l + 1, nums.len() - 1);
            loop {
                if m >= r {
                    break;
                }
                let sum = nums[l] + nums[m] + nums[r];
                if (target - sum).abs() < (target - result).abs() {
                    result = sum;
                }
                if sum < target {
                    m += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    return sum;
                }
            }
        }
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums = vec![0, 3, 97, 102, 200];
        let target = 300;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1, 2];
        let target = 0;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 4, 7, 8, 9];
        let target = 15;
        let result = Solution::three_sum_closest(nums, target);
        assert_eq!(result, 15);
    }
}
