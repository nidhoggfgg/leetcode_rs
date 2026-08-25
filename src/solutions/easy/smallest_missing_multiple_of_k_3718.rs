/*
 * @lc app=leetcode.cn id=3718 lang=rust
 *
 * [3718] Smallest Missing Multiple of K
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = 0_u128;
        for x in nums {
            if x % k == 0 {
                if x / k >= 128 {
                    continue;
                }
                bits |= 1 << (x / k - 1);
            }
        }

        (bits.trailing_ones() as i32 + 1) * k
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![
            55, 60, 59, 66, 71, 40, 22, 67, 79, 65, 40, 76, 27, 37, 87, 20, 88,
        ];
        let k = 1;
        assert_eq!(Solution::missing_multiple(nums, k), 1);
    }
}
