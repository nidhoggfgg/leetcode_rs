/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 3Sum
 */


use super::Solution;

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = nums.len();
        nums.sort_unstable();
        if len < 3 { return result; }
        for left in 0..len-2 {
            if left > 0 && nums[left] == nums[left-1] {continue;}
            let mut mid = left + 1;
            let mut right = len - 1;
            while mid < right {
                let v = nums[left] + nums[mid] + nums[right];
                match v.cmp(&0) {
                    Ordering::Less => mid += 1,
                    Ordering::Equal => {
                        result.push(vec![nums[left], nums[mid], nums[right]]);
                        while mid < right && nums[mid] == nums[mid+1] { mid+=1; }
                        while right > mid && nums[right] == nums[right-1] { right-=1; }
                        mid += 1;
                        right -= 1;
                    }
                    Ordering::Greater => right -= 1,
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
    fn test_example() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(Solution::three_sum(nums), [[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn test_all_zeroes_are_deduped() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(Solution::three_sum(nums), [[0, 0, 0]]);
    }

    #[test]
    fn test_too_short() {
        assert_eq!(Solution::three_sum(vec![0, 1]), Vec::<Vec<_>>::new());
    }
}
