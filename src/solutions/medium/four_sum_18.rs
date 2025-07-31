/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 4Sum
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let mut result = Vec::new();
        if nums.len() < 4 {
            return result;
        }
        nums.sort_unstable();

        let mut seen = HashSet::new();
        for l in 0..nums.len() - 3 {
            for m1 in l + 1..nums.len() - 2 {
                let (mut m2, mut r) = (m1 + 1, nums.len() - 1);
                let t = target - nums[l] as i64 - nums[m1] as i64;
                while m2 < r {
                    let sum = (nums[m2] + nums[r]) as i64;
                    if sum == t {
                        let tmp = (nums[l], nums[m1], nums[m2], nums[r]);
                        if !seen.contains(&tmp) {
                            result.push(vec![nums[l], nums[m1], nums[m2], nums[r]]);
                            seen.insert(tmp);
                        }

                        // nums[m2] == .... == nums[r1]
                        if nums[m2] == nums[r] {
                            break;
                        }

                        // nums[m2] == nums[m2 + 1] == .... != some != ... nums[r]
                        let mut left_count = 1;
                        while m2 + left_count < r
                            && nums[m2 + left_count - 1] == nums[m2 + left_count]
                        {
                            left_count += 1;
                        }
                        let mut right_count = 1;
                        while r - right_count > m2 && nums[r] == nums[r - right_count] {
                            right_count += 1;
                        }
                        m2 += left_count;
                        r -= right_count;

                        continue;
                    }
                    if sum < t {
                        m2 += 1;
                    } else {
                        r -= 1;
                    }
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
        let nums = vec![-2, -1, 0, 0, 1, 2];
        let target = 0;
        assert_eq!(
            Solution::four_sum(nums, target),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        )
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        assert_eq!(Solution::four_sum(nums, target), [[2, 2, 2, 2]])
    }

    #[test]
    fn test_3() {
        let nums = vec![-2, -1, -1, 1, 1, 2, 2];
        let target = 0;
        assert_eq!(
            Solution::four_sum(nums, target),
            [[-2, -1, 1, 2], [-1, -1, 1, 1]]
        );
    }

    #[test]
    fn test_4() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        assert_eq!(Solution::four_sum(nums, target), Vec::<Vec<_>>::new())
    }
}
