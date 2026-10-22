/*
 * @lc app=leetcode.cn id=3942 lang=rust
 *
 * [3942] Minimum Operations to Sort a Permutation
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ord = 0;
        let mut oi = 0;
        let mut unord = 0;
        let mut ui = 0;
        let n = nums.len();

        if n <= 1 {
            return 0;
        }

        for i in 0..n - 1 {
            if nums[i] < nums[i + 1] {
                ord += 1;
                oi = i;
            }
            if nums[i] > nums[i + 1] {
                unord += 1;
                ui = i;
            }
        }

        if nums[0] < nums[n - 1] {
            unord += 1;
            ui = n - 1;
        }
        if nums[0] > nums[n - 1] {
            ord += 1;
            oi = n - 1;
        }

        if ord >= 2 && unord >= 2 {
            return -1;
        }

        if unord == 1 {
            let k = (ui + 1) % n;
            return k.min(n - k + 2) as i32;
        }

        if ord == 1 {
            let k = n - 1 - oi;
            return (1 + k.min(n - k)) as i32;
        }

        -1
    }
}
// @lc code=end
