/*
 * @lc app=leetcode.cn id=2958 lang=rust
 *
 * [2958] Length of Longest Subarray With at Most K Frequency
 */
pub struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut counts = HashMap::new();
        let mut ans = 0;
        let mut j = 0;
        counts.insert(nums[0], 1);
        for i in 0..n {
            if *counts.get(&nums[j]).unwrap() <= k {
                while j < n - 1 {
                    j += 1;
                    counts.entry(nums[j]).and_modify(|x| *x += 1).or_insert(1);
                    if *counts.get(&nums[j]).unwrap() > k {
                        break;
                    }
                }
                if j == n - 1 {
                    let v = if *counts.get(&nums[j]).unwrap() > k {
                        j - i
                    } else {
                        j - i + 1
                    };
                    return v.max(ans) as i32;
                } else {
                    ans = ans.max(j - i);
                }
            }
            counts.entry(nums[i]).and_modify(|x| *x -= 1);
        }

        ans as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        assert_eq!(Solution::max_subarray_length(nums, k), 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 2, 2];
        let k = 1;
        assert_eq!(Solution::max_subarray_length(nums, k), 2);
    }
}
