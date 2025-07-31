/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 4Sum
 */

use super::Solution;

// @lc code=start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let mut result = Vec::new();
        if nums.len() < 4 {
            return result;
        }
        nums.sort_unstable();
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();

        let len = nums.len();
        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let (mut l, mut r) = (j + 1, len - 1);
                let t = target - nums[i] - nums[j];
                let max = nums[r] + nums[r - 1];
                let min = nums[l] + nums[l + 1];
                if max < t || min > t {
                    continue;
                }

                while l < r {
                    let sum = nums[l] + nums[r];
                    if sum < t {
                        l += 1;
                    } else if sum > t {
                        r -= 1;
                    } else {
                        result.push(vec![
                            nums[i] as i32,
                            nums[j] as i32,
                            nums[l] as i32,
                            nums[r] as i32,
                        ]);

                        // nums[m2] == .... == nums[r1]
                        if nums[l] == nums[r] {
                            break;
                        }

                        // nums[m2] == nums[m2 + 1] == .... != some != ... nums[r]
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
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
